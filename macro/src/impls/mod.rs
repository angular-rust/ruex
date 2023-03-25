#[allow(unused_imports)]
use proc_macro::TokenStream;
use serde::Deserialize;

mod compose;
pub(crate) use self::compose::*;

mod contract_aspect;
pub(crate) use self::contract_aspect::*;

mod decorate;
pub(crate) use self::decorate::*;

mod delegate;
pub(crate) use self::delegate::*;

mod enums;
pub(crate) use self::enums::*;

mod mock;
pub(crate) use self::mock::*;

mod register;
pub(crate) use self::register::*;

mod utils;

#[derive(Debug, Default, Clone, PartialEq, Eq, Deserialize)]
struct MainConfig {
    #[serde(default)]
    backtrace: BacktraceConfig,
    #[serde(default)]
    log: LogConfig,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Deserialize)]
struct LogConfig {
    #[serde(default)]
    level: LogLevel,
    #[serde(default)]
    output: LogOutput,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Deserialize)]
enum LogOutput {
    #[default]
    StdOut,
    Syslog,
    File(String),
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Deserialize)]
enum LogLevel {
    Trace,
    Debug,
    #[default]
    Info,
    Warn,
    Error,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Deserialize)]
struct BacktraceConfig {
    #[serde(default)]
    level: BactraceLevel,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Deserialize)]
enum BactraceLevel {
    None,
    #[default]
    Short,
    Full,
}

#[cfg(not(test))] // Work around for rust-lang/rust#62127
pub(crate) fn main(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = proc_macro2::TokenStream::from(attr);
    let config = if !attr.is_empty() {
        match serde_tokenstream::from_tokenstream::<MainConfig>(&attr) {
            Ok(config) => config,
            Err(err) => return err.to_compile_error().into(),
        }
    } else {
        MainConfig::default()
    };

    let input: syn::ItemFn = syn::parse2(item.into()).unwrap();

    let backtrace_level = match config.backtrace.level {
        BactraceLevel::None => quote::quote!("0"),
        BactraceLevel::Short => quote::quote!("1"),
        BactraceLevel::Full => quote::quote!("full"),
    };

    let log_level = match config.log.level {
        LogLevel::Trace => quote::quote!("trace"),
        LogLevel::Debug => quote::quote!("debug"),
        LogLevel::Info => quote::quote!("info"),
        LogLevel::Warn => quote::quote!("warn"),
        LogLevel::Error => quote::quote!("error"),
    };

    let log_initialization = match config.log.output {
        LogOutput::StdOut => {
            quote::quote!({
                use logging::{
                    colors::{Color, ColoredLevelConfig},
                    Dispatch, LevelFilter,
                };

                let colors = ColoredLevelConfig::new()
                    .debug(Color::Blue)
                    .info(Color::Green)
                    .warn(Color::Yellow)
                    .error(Color::Red)
                    .trace(Color::BrightBlack);

                Dispatch::new()
                    .format(move |out, message, record| {
                        out.finish(format_args!(
                            "[{} {:5} {}] {}",
                            format_rfc3339(std::time::SystemTime::now()),
                            colors.color(record.level()),
                            record.target(),
                            message
                        ))
                    })
                    // .level(LevelFilter::Debug)
                    .chain(std::io::stdout())
                    .apply()
                    .unwrap();
            })
        }
        LogOutput::Syslog => quote::quote!({
            use logging::{Dispatch, LevelFilter};

            const CARGO_PKG_NAME: &'static str = env!("CARGO_PKG_NAME");

            let formatter = logging::syslog::Formatter3164 {
                facility: logging::syslog::Facility::LOG_USER,
                hostname: None,
                process: CARGO_PKG_NAME.to_owned(),
                pid: 0,
            };

            Dispatch::new()
                // Perform allocation-free log formatting
                .format(move |out, message, record| {
                    out.finish(format_args!(
                        "[{} {:5} {}] {}",
                        format_rfc3339(std::time::SystemTime::now()),
                        record.level(),
                        record.target(),
                        message
                    ))
                })
                .chain(logging::syslog::unix(formatter).unwrap())
                .apply()
                .unwrap();
        }),
        LogOutput::File(filename) => quote::quote!({
            use logging::{
                colors::{Color, ColoredLevelConfig},
                Dispatch, LevelFilter,
            };

            Dispatch::new()
                // Perform allocation-free log formatting
                .format(move |out, message, record| {
                    out.finish(format_args!(
                        "[{} {:5} {}] {}",
                        format_rfc3339(std::time::SystemTime::now()),
                        record.level(),
                        record.target(),
                        message
                    ))
                })
                .chain(logging::log_file(#filename).unwrap())
                .apply()
                .unwrap();
        }),
    };

    let syn::ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = input;

    let stmts = &block.stmts;
    let output = quote::quote! {
        #(#attrs)* #vis #sig {
            if cfg!(debug_assertions) {
                use std::env;

                if env::var("RUST_BACKTRACE").is_err() {
                    env::set_var("RUST_BACKTRACE", #backtrace_level);
                }

                if env::var("RUST_LOG").is_err() {
                    env::set_var("RUST_LOG", #log_level);
                }

                panic::backtrace::install();
            } else {
                // Build the metadata from the build environment
                const PACKAGE_METADATA: panic::PackageMetadata<'static> = panic::PackageMetadata {
                    pkg_name: env!("CARGO_PKG_NAME"),
                    crate_name: env!("CARGO_CRATE_NAME"),
                    version: env!("CARGO_PKG_VERSION"),
                    repository: env!("CARGO_PKG_REPOSITORY"),
                };

                // Append the new panic handler
                panic::handler(panic::suggest_issue_tracker, PACKAGE_METADATA);
            }

            #log_initialization

            #(#stmts)*
        }
    };

    output.into()
}
