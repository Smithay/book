# General principles

The Wayland protocol follows a client-server architecture: the Wayland compositor[^compositor]
listens on an UNIX socket, to which client apps then connect in order to present their graphical
interface. This UNIX connection transports information in both directions: the client uses
it to tell the server what it wants to display, and the server tells the client about user
actions (such as keyboard or pointer input). 

[^compositor]: The terms "compositor" and "server" are used interchangeably in the Wayland ecosystem.

The first step a client will perform is finding Wayland socket and try to connect to it.

The server tells to clients about the Wayland socket using the `WAYLAND_DISPLAY` environment
variable. The listening socket is placed at `$XDG_RUNTIME_DIR/$WAYLAND_DISPLAY`. This will
typically be a path like `/run/user/1000/wayland-0`.

There are other ways the server can tell the client where the Wayland listening socket is located, but these methods are not relevant yet.

The wayland-client crate provides a simple way to do above steps using the [`Connection::connect_to_env()`](https://docs.rs/wayland-client/*/wayland_client/struct.Connection.html#method.connect_to_env)
function:

```rust,no_run
{{#rustdoc_include intro.rs:connect_to_env}}
```

The `Connection` object is what your program uses to coordinate all communication with the wayland server.
The connection may be used to create other objects in order to initialize your program's state. Before get to application setup, let's take some time to understand the Wayland protocol a bit more in the next chapter.
