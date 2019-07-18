use kvs::command_prelude::{App, Arg, SubCommand};

pub fn cli() -> App {
    SubCommand::with_name("get")
        .about("Get the string value of a given string key")
        .arg(Arg::with_name("KEY").help("A string key").required(true))
}
