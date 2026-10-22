use teloxide::types::Me;
use tracing::info;

pub fn run(me: &Me) {
    info!(target: "client", "Successfully started as '@{}'", me.username());
}
