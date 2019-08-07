use std::env;

use kvs::command_prelude::{App, Arg, SubCommand};
use kvs::Result;
use kvs::{KvOpts, KvStore};

pub fn cli() -> App {
    SubCommand::with_name("get")
        .about("Get the string value of a given string key")
        .arg(Arg::with_name("KEY").help("A string key").required(true))
}

pub fn exec(key: String) -> Result<Option<String>> {
    KvStore::open_with_opts(env::current_dir()?, KvOpts {})?.get(key)
}
