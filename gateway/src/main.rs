use gateway::setup;

use tracing::info;

fn main() {
    setup();
    println!("gateway");
    let port = 111;
    info!(port, "22"=11,"33"=22,"connected to {:?}", "222");
}

