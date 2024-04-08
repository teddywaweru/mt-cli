use crate::sockets::ConnectionSockets;
pub struct IndicatorData {
    indi_1: f32,
    indi_2: f32,
    indi_3: f32,
    indi_4: f32,
}

impl IndicatorData {
    fn new() -> Self {
        IndicatorData {
            indi_1: 32.0,
            indi_2: 32.0,
            indi_3: 32.0,
            indi_4: 32.0,
        }
    }
    fn get() -> Result<Self, zmq::Error> {
        let sockets = ConnectionSockets::initialize()?;
        sockets.connect()?;
        let data = "TRADE;GET_INDICATOR_DATA";
        sockets.request.send(data, 0)?;

        // Parse Indicator data to save
        let data = sockets.receive()?;
        // let data = self::parse_indicator_data(data);
        todo!()
    }
    fn parse_indicator_data(data: String) -> Self {
        todo!()
    }
    pub fn get_historical_data(
        instrument: &str,
        timeframe: u32,
        duration: u32,
    ) -> Result<Vec<Self>, zmq::Error> {
        for _ in 0..duration {
            IndicatorData::get();
        }
        todo!()
    }
}
