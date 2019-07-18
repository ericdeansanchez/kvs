use kvs::command_prelude::{App, Arg, SubCommand};

pub fn cli() -> App {
    SubCommand::with_name("rm")
        .about("Remove a given key")
        .arg(Arg::with_name("KEY").help("A string key").required(true))
}
