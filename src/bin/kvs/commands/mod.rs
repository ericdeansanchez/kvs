use kvs::command_prelude::*;

pub fn all_sub_commands() -> Vec<App> {
    vec![get::cli(), set::cli(), remove::cli()]
}

pub mod get;
pub mod remove;
pub mod set;
