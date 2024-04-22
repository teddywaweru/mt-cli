# Purpose

# Requirements

Install MetaTrader4 or MetaTrader5(not being actively developed currently)
Get mt4/5-zmq library
Get Expert Advisor

# Roadmap

Implement a testing section
Implement data collection for symbols
Implement an auto trading sequence
Implement a frontend UI for project
Rewrite Expert Advisor to adhere to json data transfer practices on serde-json crate

Data Structures: IndicatorData, TickData, Trades, MessagesToMT4, MessagesFromMT4

Task:
Run algorunner tr log sim strategy

- Collect data for ticks OHLC
  Consider the time element for the data points
- Collect data for indicators
- Impl strategy, searching for favouring opportunities
- Calculate Trade parameters for trades: lot size, SL, TP
- Monitor positions and Close if targets are reached.
- Create records of trades over time.

Do we have the data elements be in terms of time? Important consideration, given that each struct is meant to represent a certain scope of time. And this would also be the layout on any DB.
o

Defining the structure of the project:
Data Structures

1. Tick Data: OHLC data for tick
2. Rates Data: bid and ask prices
3. Indicator Data: data for indicator/indicators..
4. zmq Connection Data: Ports, and port types
   /// account balance /
   /// active trades
   /// trade margins
   /// test indicator
   /// find trading trend
   /// subscribe to symbol
   /// place trade

---

Project focuses on calculating parameters for trades I would enter over time.
SHOULD WORK ON BOTH LINUX AND WINDOWS

## Minimum viable product workflow

1. User provides a currency pair(string)
2. Program to fetch data on the currency pair to calculate(requires connecting to a platform, whichever that may be)
3. Program calculates basic parameter values for a feasible trade to enter for the currency ie. current price point, pips to a TP, pips to an SL,
4. Provide the calculated value to the user on the terminal

## After this...

1. Include altering the values to be used during the calculation, percentage risk to use, lot size to use

## After this...

1. GUI

## Functionality:

Calculation of Lot Sizes
Calculatoin of Stop Losses and Take Profits

## Essential:

To work on both UNIX and Windows Systems

## Auxillary:

Create new trades on whichever platform that is ultimately going to be connected to the calculator

## Nice to Have

Notifications on Price Hits?
Reporting on trades?
Capture data from external sources ie. price values? News events?

## TODO

[] Proper note taking on Obsidian, so ideas can be linked.

1. Determine the aspects to connect to? Do we have a Windows server running the MT5 application?
   Options for setting up the server.
2. Configure the design architecture for the program
   Based on functionality-
   Calculator,
   Incoming data(from user)
   Outgoing data(from user)
   Form of display: tui, gui,
   Incoming data(fetched from external sources)
   Outgoing data(fetched from external sources)
