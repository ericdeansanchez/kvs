pub use clap::{AppSettings, Arg, ArgMatches, SubCommand};

/// Type alias for a `clap::App`.
pub type App = clap::App<'static, 'static>;
