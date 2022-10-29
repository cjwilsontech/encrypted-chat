use actix::prelude::*;
use rand::{self, rngs::ThreadRng, Rng};
use serde::Serialize;
use std::collections::HashMap;

pub struct ChatManager {
    sessions: HashMap<usize, Recipient<ChatMessage>>,
    rng: ThreadRng,
}

impl ChatManager {
    pub fn new() -> ChatManager {
        ChatManager {
            rng: rand::thread_rng(),
            sessions: HashMap::new(),
        }
    }
}

impl Actor for ChatManager {
    type Context = Context<Self>;
}

impl Handler<Connect> for ChatManager {
    type Result = usize;

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        let client_id = self.rng.gen::<usize>();

        println!("Joined (ID: {})", client_id);

        self.sessions.insert(client_id, msg.client_addr);
        client_id
    }
}

impl Handler<Disconnect> for ChatManager {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        println!("Disconnected (ID: {})", &msg.client_id);
        self.sessions.remove(&msg.client_id);
    }
}

impl Handler<ChatMessage> for ChatManager {
    type Result = ();

    fn handle(&mut self, msg: ChatMessage, _: &mut Context<Self>) {
        for (id, session) in &self.sessions {
            if *id != msg.client_id {
                session.do_send(msg.clone());
            }
        }
    }
}

#[derive(Message)]
#[rtype(usize)]
pub struct Connect {
    pub client_addr: Recipient<ChatMessage>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub client_id: usize,
}

#[derive(Clone, Message, Serialize)]
#[rtype(result = "()")]
pub struct ChatMessage {
    pub client_id: usize,
    pub message: String,
}
