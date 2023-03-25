//! The terminal color and links support.
//!
//!
//! ```
//! # #![allow(unused_imports)]
//! use ruex::utils::terminal::*;
//! ```

use std::fmt;

pub use colored::*;

pub use atty::Stream;

/// Returns true if the current terminal, detected through various environment
/// variables, is known to support hyperlink rendering.
pub fn supports_hyperlinks() -> bool {
    // Hyperlinks can be forced through this env var.
    if let Ok(arg) = std::env::var("FORCE_HYPERLINK") {
        return arg.trim() != "0";
    }

    if std::env::var("DOMTERM").is_ok() {
        // DomTerm
        return true;
    }

    if let Ok(version) = std::env::var("VTE_VERSION") {
        // VTE-based terminals above v0.50 (Gnome Terminal, Guake, ROXTerm, etc)
        if version.parse().unwrap_or(0) >= 5000 {
            return true;
        }
    }

    if let Ok(program) = std::env::var("TERM_PROGRAM") {
        if matches!(
            &program[..],
            "Hyper" | "iTerm.app" | "terminology" | "WezTerm"
        ) {
            return true;
        }
    }

    if let Ok(term) = std::env::var("TERM") {
        // Kitty
        if matches!(&term[..], "xterm-kitty") {
            return true;
        }
    }

    // Windows Terminal and Konsole
    std::env::var("WT_SESSION").is_ok() || std::env::var("KONSOLE_VERSION").is_ok()
}

/// Returns true if `stream` is a TTY, and the current terminal
/// [supports_hyperlinks].
pub fn on(stream: atty::Stream) -> bool {
    (std::env::var("FORCE_HYPERLINK").is_ok() || atty::is(stream)) && supports_hyperlinks()
}

/// A clickable link component.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Link<'a> {
    /// link identifier
    pub id: &'a str,
    /// link text
    pub text: &'a str,
    /// link url
    pub url: &'a str,
}

impl<'a> Link<'a> {
    /// Create a new link with a name and target url.
    pub fn new(text: &'a str, url: &'a str) -> Self {
        Self { text, url, id: "" }
    }

    /// Create a new link with a name, a target url and an id.
    pub fn with_id(text: &'a str, url: &'a str, id: &'a str) -> Self {
        Self { text, url, id }
    }
}

impl fmt::Display for Link<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.id.is_empty() {
            write!(
                f,
                "\u{1b}]8;id={};{}\u{1b}\\{}\u{1b}]8;;\u{1b}\\",
                self.id, self.url, self.text
            )
        } else {
            write!(
                f,
                "\u{1b}]8;;{}\u{1b}\\{}\u{1b}]8;;\u{1b}\\",
                self.url, self.text
            )
        }
    }
}
