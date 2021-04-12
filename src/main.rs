//use futures::{future, pin_mut, StreamExt};
use log::{debug, info, trace, warn};

use async_std::task;

mod bot;
pub mod captcha;
pub mod fasttalk;
pub mod protocol;

async fn run() {
    info!("Initializing bot");
    let mut bot = bot::Bot::connect(
        "wss://ak7oqfc2u4qqcu6i.d.nsrv.cloud:5002/?a=1".to_owned(),
        "Aspect".to_owned(),
    )
    .await;
    info!("Bot is ready");
    // listen for events
}

fn main() {
    env_logger::init();

    task::block_on(run())
}
