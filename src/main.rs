#![allow(unused)]

use clap::{Parser, Subcommand};
use zmq;
//add following packages:
//1. Color-eyre
//2. iRust
//3. Bacon
//4. Tracing
//5. SQLx
//6. Clap
mod account;
mod cli_args;
mod indicator;
mod parse_data;
mod sockets;
mod test_algorithm;
mod tick;

use cli_args::Args;

fn main() {
    // const DEFAULT_INSTRUMENT: String = get_string();
    // TODO
    // Implement args: make request, check response,
    // Implement query for historical data
    // Implement query for subscriptions
    // Implement query for order creation, and tracking
    // Try implement using Router
    //
    let args = Args::parse().run();
}
