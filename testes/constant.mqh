//+------------------------------------------------------------------+
//|                                                           testes |
//|                                  Copyright 2023, MetaQuotes Ltd. |
//|                                             https://www.mql5.com |
//+------------------------------------------------------------------+
string PROJECT_NAME = "ZMQ_MT5_Server";
string ZEROMQ_PROTOCOL = "tcp";
string HOSTNAME = "*";
int PUSH_PORT = 32768;
int PULL_PORT = 32769;
int PUB_PORT = 32770;
int MILLISECOND_TIMER = 1;
int MaximumSlippage = 3;


// CREATE ZeroMQ Context
Context context(PROJECT_NAME);

// CREATE ZMQ_PUSH SOCKET
Socket pushSocket(context, ZMQ_PUSH);

// CREATE ZMQ_PULL SOCKET
Socket pullSocket(context, ZMQ_PULL);

// CREATE ZMQ_PUB SOCKETMT5|Expert!0
Socket pubSocket(context, ZMQ_PUB);



//+------------------------------------------------------------------+
