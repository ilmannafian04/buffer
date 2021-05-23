// use std::collections::{HashMap, HashSet};
//
// use actix::prelude::*;
// use rand::rngs::ThreadRng;
//
// #[derive(Message)]
// #[rtype(result = "()")]
// pub struct Message(pub String);
//
// #[derive(Message)]
// #[rtype(usize)]
// pub struct Connect {
//     pub addr: Recipient<Message>,
// }
//
// #[derive(Message)]
// #[rtype(result = "()")]
// pub struct Disconnect {
//     pub id: usize,
// }
//
// #[derive(Message)]
// #[rtype(result = "()")]
// pub struct ClientMessage {
//     pub id: usize,
//     pub message: String,
//     pub room: String,
// }
//
// pub struct ChatServer {
//     session: HashMap<usize, Recipient<Message>>,
//     rooms: HashMap<String, HashSet<usize>>,
//     rng: ThreadRng,
// }
//
// impl Actor for ChatServer {
//     type Context = Context<Self>;
// }
