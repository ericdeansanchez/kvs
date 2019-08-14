use std::io::{self, Write};
use std::process::exit;

use kvs::Result;

mod cli;
mod commands;

fn main() -> Result<()> {
    // run the cli app
    Ok(run(cli::app())?)
}

/// Executes a cli app. This function parses the command line arguments and
/// maps a given command to _its_ executor.
fn run(app: clap::App<'static, 'static>) -> Result<()> {
    match app.get_matches().subcommand() {
        ("get", Some(args)) => get(args),
        ("rm", Some(args)) => remove(args),
        ("set", Some(args)) => set(args),
        _ => {
            exit(1);
        }
    }
}

fn get(arg_matches: &clap::ArgMatches) -> Result<()> {
    let key = arg_matches
        .value_of("KEY")
        .map(String::from)
        .expect("KEY argument missing");

    if let Some(value) = commands::get::exec(key)? {
        io::stdout().write_fmt(format_args!("{}", value))?;
    } else {
        io::stdout().write(b"Key not found")?;
    }
    Ok(())
}

fn set(arg_matches: &clap::ArgMatches) -> Result<()> {
    let key = arg_matches
        .value_of("KEY")
        .map(String::from)
        .expect("KEY argument missing");

    let value = arg_matches
        .value_of("VALUE")
        .map(String::from)
        .expect("VALUE argument missing");

    commands::set::exec(key, value)
}

fn remove(arg_matches: &clap::ArgMatches) -> Result<()> {
    let key = arg_matches
        .value_of("KEY")
        .map(String::from)
        .expect("KEY argument missing");

    match commands::remove::exec(key) {
        Ok(()) => {}
        Err(_) => {
            io::stdout().write(b"Key not found")?;
            exit(2);
        }
    }
    Ok(())
}
