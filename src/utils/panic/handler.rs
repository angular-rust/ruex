//! Here lives our custom panic handler and its support code
//!
//! ## Panic handler chaining
//!
//! Rust already has a pretty good panic handling mechanism that I see no need to reinvent.
//! As a way to keep this functionality while still being able to print out a "Report this" message,
//! we can simply chain the handlers together.
//!
//! The reason I am not making use of
//! [`std::panic::update_hook`](https://doc.rust-lang.org/std/panic/fn.update_hook.html)
//! is both because it is a nightly feature. Maybe this can be added in the future.

use std::panic::PanicInfo;

use url::Url;

use crate::utils::terminal::{Colorize, Link, Stream};

use super::validate::validate_repository;

/// Contains metadata that we pull from the client crate with a macro
#[derive(Debug)]
pub struct PackageMetadata<'a> {
    /// The client package name
    pub pkg_name: &'a str,
    /// The client crate version (bin or example)
    pub crate_name: &'a str,
    /// The client crate version
    pub version: &'a str,
    /// This may or may not be a repository URL.
    /// Requires validation through `validate_repository()`
    pub repository: &'a str,
}

/// Append a panic handler to the end of the chain of panic handlers
///
/// This function will execute whatever the existing panic handler is *before* executing the new one.
/// Metadata is required to inform our crash reporter about the issue tracker URL.
pub fn handler<F>(hook_fn: F, metadata: PackageMetadata<'static>)
where
    F: Fn(&PanicInfo<'_>, &PackageMetadata) + Sync + Send + 'static,
{
    // Get the current panic handler
    let prev_hook = std::panic::take_hook();

    // Create a new panic handler that chains our custom panic handler with the previous one
    std::panic::set_hook(Box::new(move |panic_info| {
        // Call the previous panic handler
        prev_hook(panic_info);

        // Call our custom panic handler
        hook_fn(panic_info, &metadata);
    }));
}

/// A panic handler that prints out a "Report this" message.
///
/// This will try to determine an issue tracker URL from the crate metadata and link it in the terminal with a pre-made message.
pub fn suggest_issue_tracker(info: &PanicInfo<'_>, metadata: &PackageMetadata) {
    // If color is disabled, we need to inform the `colored` crate before we start printing
    if cfg!(not(feature = "color")) {
        colored::control::set_override(false);
    }

    // We have access to the repo metadata.
    // If we have not found a supported repository, we cannot proceed with linking an issue tracker.
    // This requires validating the repository URL.
    let mut report_url = None;

    if !&metadata.repository.is_empty() {
        if let Some(provider) = validate_repository(&Url::parse(metadata.repository).unwrap()) {
            report_url = Some(provider.build_issue_url(info, metadata));
        }
    }

    // Print the message
    println!(
        "{}",
        format!(
            "\n---------------------- {} ----------------------\n",
            "Crash Detected".bold()
        )
        .red()
    );

    if let Some(url) = report_url {
        // Print a message with a link to the issue tracker
        println!(
            "{}",
            "This application has issue tracker support enabled.".italic()
        );
        println!(
            "{}",
            "Click the link below to report this crash to the developers.".bold()
        );

        // If this terminal supports clickable links, use one of those
        // NOTE: VSCode both reports having clickable link support *and* does not let you click on the link
        if crate::utils::terminal::on(Stream::Stdout)
            && !std::env::var("TERM_PROGRAM")
                .unwrap_or("unknown".to_string())
                .contains("vscode")
        {
            let link = Link::new("Report Crash", url.as_str());
            println!("\n[{}]", link.to_string().cyan().bold());
        } else {
            // Otherwise, just print the URL
            println!("\n{}", url.to_string().cyan().bold());
        }
    } else {
        // Well, this is awkward. Someone is using this crate without adding a repository key to their Cargo.toml.
        if cfg!(debug_assertions) {
            // If we are in a debug build. Warn the developer.
            println!(
                "{}",
                "This application has issue tracker support enabled.".italic()
            );
            println!("However, it was not possible to determine the repository URL.");
            println!(
                "Please add a `{}` key to your {}.",
                "repository".bright_green().bold(),
                "Cargo.toml".cyan().bold()
            );
            println!(
                "{}",
                "\nThere is also a chance your repository service is not supported\nYou can request support at: https://github.com/ewpratten/crashreport-rs".italic()
            );
        } else {
            // Just tell the user something went wrong
            println!(
                "{}",
                "This application has issue tracker support enabled.".italic()
            );
            println!("However, it was not possible to determine the issue tracker URL.");
        }
    }

    println!(
        "{}",
        "\n------------------------------------------------------------\n".red()
    )
}
