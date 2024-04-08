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
#[derive()]
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
    pub fn initialize(// ctx: &zmq::Context,
               // states: Vec<zmq::SocketType>,
    ) -> Result<Self, zmq::Error> {
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
    fn connect(&self) -> Result<&Self, zmq::Error> {
        self.response.connect("tcp://127.0.0.1:32769")?;
        self.request.connect("tcp://127.0.0.1:32768")?;
        self.subscribe.connect("tcp://127.0.0.1:32770")?;

        Ok(self)
    }

    pub fn request(&self, data: &str, flag: i32) -> Result<&Self, zmq::Error> {
        self.request.send(data, flag)?;

        Ok(self)
    }

    pub fn receive(&self) -> Result<String, zmq::Error> {
        // let mut msg: zmq::Message;
        let flag = 0;
        let response = match self.response.recv_string(flag)? {
            Ok(response) => response,
            Err(e) => {
                panic!("Failed to receive a valid message: {:?}", e)
            }
        };

        Ok(response)
    }
}
