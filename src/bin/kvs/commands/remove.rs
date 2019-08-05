use kvs::command_prelude::{App, Arg, SubCommand};
use kvs::Result;

pub fn cli() -> App {
    SubCommand::with_name("rm")
        .about("Remove a given key")
        .arg(Arg::with_name("KEY").help("A string key").required(true))
}

pub fn exec(key: String) -> Result<()> {
    Ok(())
}
