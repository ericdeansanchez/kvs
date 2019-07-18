//! # Generates the top-level cli.
use kvs::command_prelude::*;

pub fn init() -> App {
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
        ])
        .subcommand(
            SubCommand::with_name("set")
                .about("Set the value of a given key")
                .arg(Arg::with_name("KEY").help("A string key").required(true))
                .arg(
                    Arg::with_name("VALUE")
                        .help("The value of the key")
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("get")
                .about("Get the value of a given key")
                .arg(Arg::with_name("KEY").help("A string key").required(true)),
        )
        .subcommand(
            SubCommand::with_name("rm")
                .about("Remove a given key")
                .arg(Arg::with_name("KEY").help("A string key").required(true)),
        );
    app
}
