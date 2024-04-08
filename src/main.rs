#![allow(unused)]

//add following packages:
//1. Color-eyre
//2. iRust
//3. Bacon
//4. Tracing
//5. SQLx
//6. Clap
mod account;
mod cli;
mod cli_args;
mod indicator;
mod mt5_bridge;
mod sockets;
mod test_algorithm;
mod tick;

use cli::run_cli;

fn main() {
    // const DEFAULT_INSTRUMENT: String = get_string();
    // TODO
    // Implement args: make request, check response,
    // Implement query for historical data
    // Implement query for subscriptions
    // Implement query for order creation, and tracking
    // Try implement using Router
    //
    //Determine if we are running a tui, gui or cli
    // Running a cli only for now
    run_cli();
}
