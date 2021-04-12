use crate::fasttalk::*;
pub enum Outbound {
    Spawn { name: String, captcha: String },
}

pub fn encode_outbound(data: Outbound) -> Vec<u8> {
    match data {
        Outbound::Spawn { name, captcha } => encode(vec![
            Block::String(packet_ids::SPAWN.to_owned()),
            Block::String(name),
        ]),
        _ => unimplemented!(),
    }
}

mod packet_ids {
    pub const SPAWN: &str = "s";
}
