use kvs::command_prelude::{App, Arg, SubCommand};
use kvs::Result;

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

pub fn exec(key: String, value: String) -> Result<()> {
    Ok(())
}
