use actix::prelude::*;
use actix_web_actors::ws;
use std::time::{Duration, Instant};

use crate::chat_manager::{self, ChatManager};

pub struct WsClientSession {
    /// Unique ID for the session.
    pub id: usize,

    /// Instant of the last successful heartbeat.
    pub hb: Instant,

    /// ChatManager actor address.
    pub chat_manager: Addr<ChatManager>,
}

impl WsClientSession {
    pub fn new(chat_manager_addr: Addr<ChatManager>) -> WsClientSession {
        WsClientSession {
            id: 0,
            hb: Instant::now(),
            chat_manager: chat_manager_addr,
        }
    }

    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                println!("Socket timed out. (ID: {})", act.id);
                act.chat_manager
                    .do_send(chat_manager::Disconnect { client_id: act.id });
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

        let addr = ctx.address();
        self.chat_manager
            .send(chat_manager::Connect {
                client_addr: addr.recipient(),
            })
            .into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(res) => act.id = res,
                    Err(_) => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> actix::Running {
        self.chat_manager
            .do_send(chat_manager::Disconnect { client_id: self.id });
        actix::Running::Stop
    }
}

impl Handler<chat_manager::Message> for WsClientSession {
    type Result = ();

    fn handle(&mut self, msg: chat_manager::Message, ctx: &mut Self::Context) {
        ctx.text(msg.0);
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
