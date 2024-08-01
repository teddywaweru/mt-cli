use zmq;

enum Ports {
    Request(i32),
    Response(i32),
    Subscribe(i32),
}

impl Default for Ports {
    fn default() -> Self {
        Ports::Request(32770)
    }
}
pub struct ConnectionSockets {
    pub request: zmq::Socket,
    pub response: zmq::Socket,
    pub subscribe: zmq::Socket,
    // request_port: i32,
    // response_port: i32,
    // subscribe_port: i32,
}
impl Default for ConnectionSockets {
    fn default() -> Self {
        let ctx: zmq::Context = zmq::Context::new();
        ConnectionSockets {
            request: ctx.socket(zmq::PUSH).unwrap(),
            response: ctx.socket(zmq::PULL).unwrap(),
            subscribe: ctx.socket(zmq::SUB).unwrap(),
        }
    }
}
impl ConnectionSockets {
    pub fn init_and_connect() -> Result<ConnectionSockets, Box<dyn std::error::Error>> {
        let sockets = ConnectionSockets::initialize()?;
        // sockets.connect()?;
        Ok(sockets)
    }
    pub fn initialize() -> Result<Self, zmq::Error> {
        let ctx: zmq::Context = zmq::Context::new();
        let subscribe = ctx.socket(zmq::SUB)?;
        let request = ctx.socket(zmq::PUSH)?;
        let response = ctx.socket(zmq::PULL)?;
        Ok(ConnectionSockets {
            request,
            response,
            subscribe,
        })
    }

    // TODO impl a config files that holds the port numbers.
    pub fn connect(&self) -> Result<&Self, zmq::Error> {
        self.response.connect("tcp://127.0.0.1:32769")?;
        self.request.connect("tcp://127.0.0.1:32768")?;
        self.subscribe.connect("tcp://127.0.0.1:32770")?;

        Ok(self)
    }

    pub fn request(&self, data: &str, flag: i32) -> &Self {
        let _ = self.connect();
        match self.request.send(data, flag) {
            Ok(_) => self,
            Err(e) => {
                panic!("Unable to send request on sockets. \n Error: {e}")
            }
        }
    }

    pub fn receive(&self) -> String {
        // let mut msg: zmq::Message;
        let flag = 0;
        let response = match self.response.recv_string(flag).unwrap() {
            Ok(response) => response,
            Err(e) => {
                panic!("Failed to receive a valid message: {:?}", e)
            }
        };

        let _ = self.disconnect();
        response
    }
    pub fn disconnect(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.response.disconnect("tcp://127.0.0.1:32769").unwrap();
        self.request.disconnect("tcp://127.0.0.1:32768")?;
        self.subscribe.disconnect("tcp://127.0.0.1:32770")?;

        Ok(())
    }
}
