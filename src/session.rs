use actix::{Actor, StreamHandler, ActorContext};
use actix_web_actors::ws;

pub struct WsClientSession {
	/// Unique ID for the session.
	pub id: usize,
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
			ws::Message::Text(_) => todo!(),
			ws::Message::Binary(_) => todo!(),
			ws::Message::Continuation(_) => todo!(),
			ws::Message::Ping(_) => todo!(),
			ws::Message::Pong(_) => todo!(),
			ws::Message::Close(_) => todo!(),
			ws::Message::Nop => todo!(),
		}
    }
}