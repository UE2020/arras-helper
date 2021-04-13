use crate::fasttalk::*;

pub enum Outbound {
    Spawn { name: String, captcha: String },
    Ping(u32),
    D(i64),
    Type(String),
    K,
}

pub fn encode_outbound(data: Outbound) -> Vec<u8> {
    match data {
        Outbound::Spawn { name, captcha } => encode(vec![
            Block::String(packet_ids::SPAWN.to_owned()),
            Block::String(name),
            Block::Number(2033.),
            Block::String(captcha),
        ]),
        Outbound::Ping(time) => encode(vec![
            Block::String(packet_ids::PING.to_owned()),
            Block::Number(time as f64),
        ]),
        Outbound::D(input) => encode(vec![
            Block::String(String::from("D")),
            Block::Number(input as f64),
        ]),
        Outbound::K => encode(vec![Block::String(String::from("k"))]),
        Outbound::Type(input) => encode(vec![
            Block::String(packet_ids::TYPE.to_owned()),
            Block::String(input),
        ]),
        _ => unimplemented!(),
    }
}

mod packet_ids {
    // outbound
    pub const SPAWN: &str = "s";
    pub const PING: &str = "p";
    pub const TYPE: &str = "T";

    // inbound
    pub const WELCOME: &str = "w";
}
