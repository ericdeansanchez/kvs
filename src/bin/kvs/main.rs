use std::process::exit;

use kvs::Result;

mod cli;
mod commands;

fn main() -> Result<()> {
    // execute the cli app
    Ok(exec(cli::app())?)
}

/// Executes a cli app. This function parses the command line arguments and
/// maps a given command to _its_ executor.
fn exec(app: clap::App<'static, 'static>) -> Result<()> {
    match app.get_matches().subcommand() {
        ("get", Some(_arg_matches)) => {
            commands::get::exec("placeholder".to_owned())?;
            Ok(())
        }
        ("rm", Some(_arg_matches)) => commands::remove::exec("placeholder".to_owned()),
        ("set", Some(_arg_matches)) => commands::set::exec("place".to_owned(), "holder".to_owned()),
        _ => {
            eprintln!("unimplemented");
            exit(1);
        }
    }
}
