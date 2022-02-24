# The Wayland protocol

The Wayland protocol can be described as an asynchronous, object oriented protocol. This means clients and
the compositor send and receive messages in order to build an abstract state machine made of objects. Wayland
as a protocol is also asynchronous in the sense that a large fraction of messages are sent unprompted and
don't expect an answer.

## Interfaces

The Wayland protocol is defined in terms of interfaces. Interfaces describe the signature of some messages.
The client and server send messages to each other that match some signature defined in an interface. The
messages the client and server send and receive are associated with a protocol object.

An interface when instantiated becomes a protocol object.

The Wayland protocol defines two types of messages:

## Requests

A request is a message that a client sends to the compositor.

This is the primary method a client will use to tell the compositor about client side changes, such as
rendering a frame or creating a new window. Requests may also be used by a client to create objects.

## Events

An event is a message that a compositor sends to the client.

Events allow the compositor to tell the client about some action, such as keyboard input, when to prepare a
new frame to present and whether a window has been resized.

## Objects

Wayland as an object oriented protocol uses objects to represent parts of the client and server state machines.

The `wayland-client` crate provides a type safe way to send requests by generating a struct to represent a
handle to a Wayland protocol object. This handle is also called a `Proxy` and implements the
[`Proxy`](https://docs.rs/wayland-client/*/wayland_client/struct.Proxy.html) trait.

For example, let's say we have an interface called `wl_foo`. A struct named `WlFoo` would be generated to
represent a handle to an object of the interface `wl_foo`. The struct also define functions for each kind of
request supported by the interface. Handles to Wayland objects are more akin to a `Arc<_>` where the inner
value is the protocol object.

The Wayland protocol is also quite stateful and often relies on clients and compositors tracking data about
the other side. `wayland-client` allows associating some data with the protocol object. The associated data is
accessible when handling events from the compositor and may be accessed as long as the object handle is
accessible (using [`Proxy::data()`](https://docs.rs/wayland-client/*/wayland_client/struct.Proxy.html#method.data)).
The data is also accessible even after the protocol object is destroyed as long as at least one
object handle is accessible.

Specifics about data associated with objects is further detailed in the next couple of chapters.

## Destructors

Dropping all handles referencing a Wayland protocol object does not destroy the protocol object.

Instead to destroy a protocol object, you must use a specialized type of request or event called a destructor.
Any handles that exist when a protocol object is destroyed become inert.

Now that we understand the parts of the protocol, we can begin creating objects and working with event queues.
