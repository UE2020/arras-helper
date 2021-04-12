use std::env;

//use futures::{future, pin_mut, StreamExt};

use async_std::io;
use async_std::prelude::*;
use async_std::task;
use async_tungstenite::async_std::connect_async;
use async_tungstenite::tungstenite::protocol::Message;
use futures::SinkExt;
use futures::StreamExt;

mod fasttalk;
mod captcha;

async fn run() {
    println!("arrasbot: Connecting...");
    let ( mut ws_stream, _) = connect_async("ws://echo.websocket.org")
        .await
        .expect("Failed to connect");
    // test
    ws_stream.send(Message::Binary(fasttalk::encode(vec![fasttalk::Block::String(String::from("Hello"))]))).await;
    println!("Recv {:?}", ws_stream.next().await.unwrap().unwrap().into_data());
    println!("arrasbot: WebSocket handshake has been successfully completed");
}

fn main() {
    task::block_on(run())
}