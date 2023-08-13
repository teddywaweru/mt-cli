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
   /// subscribe to instrument
   /// place trade
