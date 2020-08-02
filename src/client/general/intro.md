# General principles

The Wayland protocol follows a client-server architecture: the Wayland compositor listens
on an UNIX socket, to which client apps then connect in order to present their graphical
interface. This UNIX connection transports information in both directions: the client uses
it to tell the server what it wants to display, and the server uses it to tell the client
about user actions (such as keyboard or pointer input). The Wayland connection is thus the
heart of a client app, and is the source of events that drives its event loop.

The server tells to clients about the Wayland socket using the `WAYLAND_DISPLAY` environment
variable. The listening socket is placed at `$XDG_RUNTIME_DIR/$WAYLAND_DISPLAY`. This will
typically be a path like `/run/user/1000/wayland-0`.

When starting your app, it will need to find the Wayland socket and connect to it. The
wayland-client crate does this for you with the
[`Display::connect_to_env()`](https://docs.rs/wayland-client/*/wayland_client/struct.Display.html#method.connect_to_env)
method. If no error occurs, this function will provide you with a `Display` object.

This object is the very heart of your Wayland program. It represents your connection to the
server, and this is from this object that everything will be further initialized. But to
understand how this objects relates to the rest of the crate, we will first need to get a
better understanding of the distinction between the protocol objects, and the Rust structs that
your program will manipulate.