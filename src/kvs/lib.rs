pub mod util;

// re-exports
pub use util::command_prelude;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
