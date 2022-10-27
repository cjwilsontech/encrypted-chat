use actix::{Actor, ActorContext, AsyncContext, StreamHandler};
use actix_web_actors::ws;
use std::time::{Duration, Instant};

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

    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                println!("Socket timed out. (ID: {})", act.id);
                ctx.stop();
                return;
            }

            ctx.ping(b"");
        });
    }
}

impl Actor for WsClientSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
    }

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

const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
