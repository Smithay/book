// ANCHOR: all

use wayland_client::Connection;

fn main() {
    // ANCHOR: connect_to_server
    let _connection = Connection::connect_to_env().unwrap();
    // ANCHOR_END: connect_to_server
}

// ANCHOR_END: all
