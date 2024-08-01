#![allow(unused)]

//add following packages:
//1. Color-eyre
//2. iRust
//3. Bacon
//4. Tracing
//5. SQLx
//6. Clap

use cli::run_cli;

fn main() {
    // TODO
    // Implement args: make request, check response,
    // Implement query for historical data
    // Implement query for subscriptions
    // Implement query for order creation, and tracking
    // Try implement using Router
    //
    //Determine if we are running a tui, gui or cli
    // Running a cli only for now

    // Check ENV variables to determine the interface and platform to be used.
    run_cli();
}
