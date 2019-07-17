//! # Generates the top-level cli.
use clap;

use clap::{App, AppSettings, Arg, ArgMatches};

pub fn init() -> App<'static, 'static> {
    let mut app = App::new(env!(stringify!(CARGO_PKG_NAME)))
        .version(env!(stringify!(CARGO_PKG_VERSION)))
        .author(env!(stringify!(CARGO_PKG_AUTHORS)))
        .about(env!(stringify!(CARGO_PKG_DESCRIPTION)))
        .settings(&[
            AppSettings::UnifiedHelpMessage,
            AppSettings::DeriveDisplayOrder,
            AppSettings::VersionlessSubcommands,
            AppSettings::AllowExternalSubcommands,
            AppSettings::SubcommandRequiredElseHelp,
        ]);
    app
}
