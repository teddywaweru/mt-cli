use crate::cli_args::Args;
use clap::{Parser, Subcommand};
pub fn run_cli() {
    let args = Args::parse().run();
}
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
