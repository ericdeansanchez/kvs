use std::process::exit;

mod cli;
mod commands;

fn main() {
    exec(cli::app());
}

fn exec(app: clap::App<'static, 'static>) {
    match app.get_matches().subcommand() {
        (stringify!(get), Some(_arg_matches)) => {
            eprintln!("unimplemented");
            exit(1);
        }
        (stringify!(rm), Some(_arg_matches)) => {
            eprintln!("unimplemented");
            exit(1);
        }
        (stringify!(set), Some(_arg_matches)) => {
            eprintln!("unimplemented");
            exit(1);
        }
        _ => {
            eprintln!("unimplemented");
            exit(1);
        }
    }
}
