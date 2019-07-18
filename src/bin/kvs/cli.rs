//! # Generates the top-level cli.
use crate::commands;
use kvs::command_prelude::*;

pub fn app() -> App {
    App::new(env!(stringify!(CARGO_PKG_NAME)))
        .version(env!(stringify!(CARGO_PKG_VERSION)))
        .author(env!(stringify!(CARGO_PKG_AUTHORS)))
        .about(env!(stringify!(CARGO_PKG_DESCRIPTION)))
        .settings(&[
            AppSettings::UnifiedHelpMessage,
            AppSettings::DeriveDisplayOrder,
            AppSettings::VersionlessSubcommands,
            AppSettings::AllowExternalSubcommands,
            AppSettings::SubcommandRequiredElseHelp,
        ])
        .subcommands(commands::all_sub_commands())
}
