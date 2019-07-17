mod cli;

fn main() {
    let app = cli::init();
    delegate_to_subcommand(app);
}

fn delegate_to_subcommand(app: clap::App<'static, 'static>) {
    let args = app.get_matches();
    /*
    match args.subcommand() {

    }
    */
}
