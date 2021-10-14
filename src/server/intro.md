# Wayland Compositors

This part of the book is dedicated to Wayland compositors, which provide a Wayland environment for Wayland applications to run inside.

The first section will be dedicated to the fundamentals of the wayland protocol on the server side, and how a
compositor handles requests from clients, sends events to clients and advertises globals to clients using `wayland-server`. 

The later sections explore using [`smithay`](https://github.com/Smithay/smithay) and how to use the useful abstractions provided to implement a full fledged compositor.
