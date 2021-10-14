## The Display

The `Display` is the heart of a Wayland compositor. A display is implicitly available to all Wayland clients
and provides a way for clients to access the registry and then globals the compositor advertises. The display
manages connections to clients and listening sockets.

The display is exposed to clients by placing a listening socket at `$XDG_RUNTIME_DIR/$WAYLAND_DISPLAY`. This
will typically be a path like `/run/user/1000/wayland-0`. The compositor is responsible for listening to this
socket for responding to client requests.
