use actix::{Actor, ActorContext, StreamHandler};
use actix_web_actors::ws;
use std::time::Instant;

pub struct WsClientSession {
    /// Unique ID for the session.
    pub id: usize,

    /// Instant of the last successful heartbeat.
    pub hb: Instant,
}

impl WsClientSession {
    pub fn new() -> WsClientSession {
        WsClientSession {
            id: 0,
            hb: Instant::now(),
        }
    }
}

impl Actor for WsClientSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, _: &mut Self::Context) {}

    fn stopping(&mut self, _: &mut Self::Context) -> actix::Running {
        actix::Running::Stop
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsClientSession {
    fn handle(&mut self, item: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let msg = match item {
            Err(_) => {
                ctx.stop();
                return;
            }
            Ok(item) => item,
        };

        match msg {
            ws::Message::Text(text) => println!("Text recieved: {}", text),
            ws::Message::Binary(_) => println!("Binary not supported."),
            ws::Message::Continuation(_) => {
                ctx.stop();
            }
            ws::Message::Ping(msg) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            ws::Message::Pong(_) => {
                self.hb = Instant::now();
            }
            ws::Message::Close(reason) => {
                ctx.close(reason);
                ctx.stop();
            }
            ws::Message::Nop => (),
        }
    }
}
