1. How to make the program compile for both Linux and Windows
2. Divide into binaries and modules
   Main Application?
   Querying whether it's a tui or gui or cli?

   Main Application Modules

- sockets
- data parsing
- calculator?

  Binaries:

- cli
- tui

  Modules:

  Undecided
  Ticks? OHLC and Volume?
  Buy and Sell?

  How to add filter parameters to getting all open trades

  Desired use case is to be able to monitor trades and prices so that they can be closed at specific prices, and the SL moved to a different point


  calc.rs => How do we calculate for new trades?
  New Trade: cli->mt5_bridge.generate -> user confirmation -> execute trade
  needs to collect info to carry out transaction: AccountInfo, InstrumentInfo, Time?, SLs and TPs...
  => Need to be able to efficiently track values without blowing up the CPU
