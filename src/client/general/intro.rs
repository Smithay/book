// ANCHOR: all
fn main() {
    // ANCHOR: connect_to_env
    use wayland_client::Connection;

    // Connecting to the compositor may fail if no compositor is available.
    let connection = Connection::connect_to_env().expect("no compositor available");
    // ANCHOR_END: connect_to_env

    drop(connection);
}
// ANCHOR_END: all
