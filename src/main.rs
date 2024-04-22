#![allow(unused)]

//add following packages:
//1. Color-eyre
//2. iRust
//3. Bacon
//4. Tracing
//5. SQLx
//6. Clap
mod cli;
mod cli_args;
mod test_algorithm;

use cli::run_cli;

fn main() {
    // const DEFAULT_SYMBOL: String = get_string();
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
