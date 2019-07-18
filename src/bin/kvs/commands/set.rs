use kvs::command_prelude::{App, Arg, SubCommand};

pub fn cli() -> App {
    SubCommand::with_name("set")
        .about("Set the value of a given key")
        .arg(Arg::with_name("KEY").help("A string key").required(true))
        .arg(
            Arg::with_name("VALUE")
                .help("The value of the key")
                .required(true),
        )
}
