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
        ("get", Some(arg_matches)) => get_from(arg_matches),
        ("rm", Some(_arg_matches)) => commands::remove::exec("placeholder".to_owned()),
        ("set", Some(arg_matches)) => set_from(arg_matches),
        _ => {
            eprintln!("unimplemented");
            exit(1);
        }
    }
}

fn get_from(arg_matches: &clap::ArgMatches) -> Result<()> {
    let key = arg_matches
        .value_of("KEY")
        .map(String::from)
        .expect("KEY argument missing");

    if let Some(value) = commands::get::exec(key)? {
        println!("{}", value);
    } else {
        println!("Key not found");
    }
    Ok(())
}

fn set_from(arg_matches: &clap::ArgMatches) -> Result<()> {
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
