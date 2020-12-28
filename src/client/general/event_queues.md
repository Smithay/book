# Event queues and filters

If the [`Display`] is the heart of your Wayland app, the [`EventQueue`] will be its
backbone. As described in the previous section, messages are sent in both directions,
and so far we only discussed how to send requests (using the methods of bare proxies),
but not how to receive events. This is done via event queues and callbacks.

## Event callbacks

Each protocol object can be assigned to a callback, which will be invoked whenever
this object receives an event. The event callback receives 3 arguments:

- a `Main<_>` proxy to the object this event is associated to
- the event itself, under the form of the `Event` enum of the interface of this object
- a mutable reference to the `DispatchData`, which is some global mutable state shared
  by the event queue (we will get into details about that when explaining the event queue)

This callback should then contain the logic of your program for handling this event. This
may require anything from just storing the new information somewhere to process it later,
to directly responding to the server by sending a request, from within the callback.

The direct way to assign a callback to an object is via the 
[`Main<_>::quick_assign()`](https://docs.rs/wayland-client/*/wayland_client/struct.Main.html#method.quick_assign)
method, which requires to be given a callback under the form of an `FnMut` closure. You can
thus capture values in the callback, but it must be a `'static` closure, so it cannot
capture references, only values (this implies it must be a `move` closure if it captures
anything).

An alternative approach is to use a [`Filter`]. You can think of filters as a `Rc<_>` wrapper
around a closure. Filters allow you to use the same closure to process events from several
objects, making state sharing easier. They are a more advanced use, and we shall come back to
them later.

## Event queues and Attached proxies

[`EventQueue`]s are the type which actually read events from the Wayland socket, and
invoke your appropriate callbacks. All protocol objects are associated at creation to an
event queue so that their events are processed.

This is where the `Attached<_>` proxies come into play. Given a `Proxy<_>`, you can attach
it to an [`EventQueue`] like so, using a event queue token:

```rust,no_run
let attached_proxy = proxy.attach(&event_queue.token());
```

Attaching a proxy only affects the proxy, not the protocol object. The effect is that whenever
a request is sent that creates a new object, this newly created object will be associated to
the event queue to which its creator proxy is attached. Given an object must always be associated
with an event queue, requests creating new object are thus only allowed from `Attached<_>`
proxies. `Main<_>` proxies behave like `Attached<_>` ones, being attached to the same queue
as the one the underlying object is associated to. Creating new objects directly from the
same event queue as their parent object is by far the most frequent case.

[`EventQueue`]s are cannot be shared between threads (they are `!Send` and `!Sync`),
this mechanism thus allow to associate different objects to different queues, and
potentially different threads, and is required for handling that in a threadsafe way.
Following that, `Attached<WlFoo>` and `Main<WlFoo>` are not `Send` either, while
`WlFoo` and `Proxy<WlFoo>` are.

## Dispatching event queues

To receive events, it is necessary to read the Wayland socket and call the appropriate callbacks.
We call this process "dispatching", and as `wayland-client` does not take the control-flow away
from you, you need to tell it to do it.

The simplest way to achieve that is with the
[`EventQueue::dispatch()`](https://docs.rs/wayland-client/*/wayland_client/struct.EventQueue.html#method.dispatch)
method. This method does three things:

- Flush the outgoing buffer (when sending a request, it is not directly written to the socket but instead
  buffered, to improve performance)
- Read pending events from the socket, if none are available block until some are received, and distribute
  these events to the internal buffer of their target event queue
- Empty the internal buffer of the event queue on which the method was called by invoking the appropriate
  callbacks

This method also requires you to provide two additional arguments:

- A mutable reference to some value `&mut T`: this reference is accessible from all callbacks invoked by this
  event queue as the `&mut DispatchData` argument. If they wish to use it, the callbacks need to handle the
  downcasting using the methods provided by [`DispatchData`], in a similar fashion as `Any`. This value is
  typically used to serve as the global state of your app.
- A fallback closure. This closure will be invoked for all events associated to an object which has not been
  assigned to a callback using `quick_assign()` or `assign()`. This is aimed at providing an alternative
  way to handle messages in a more centralized fashion, but at the moment the API is still very limited.
  You can check [this Github issue](https://github.com/Smithay/wayland-rs/issues/287) to follow its evolution.

If most of your logic resides in callbacks, the main loop of your app can thus be as simple as:

```rust,no_run
loop {
    let ret = event_queue.dispatch(
        &mut global_state,
        |_,_,_| panic!("An event was received not assigned to any callback!")
    );
    if let Err(e) = ret {
        // Some error was returned, this means that the Wayland connection is lost
        // most of the time there is nothing more to do than print a nice error message
        // and exit
    }
}
```

If you need a more complex handling of the dispatching (in the case of a multithreaded app, or
if you have more than one source of events to handle), the three steps (flushing the outgoing buffer,
reading the socket, and dispatching the events) can be done independently using respectively
[`Display::flush()`](https://docs.rs/wayland-client/*/wayland_client/struct.Display.html#method.flush),
[`EventQueue::prepare_read()`](https://docs.rs/wayland-client/*/wayland_client/struct.EventQueue.html#method.prepare_read), and
[`EventQueue::dispatch_pending()`](https://docs.rs/wayland-client/*/wayland_client/struct.EventQueue.html#method.dispatch_pending).

Having this in mind, let's see in the next part how to proceed to the initial setup of a Wayland app.


[`Display`]: https://docs.rs/wayland-client/*/wayland_client/struct.Display.html
[`EventQueue`]: https://docs.rs/wayland-client/*/wayland_client/struct.EventQueue.html
[`Filter`]: https://docs.rs/wayland-client/*/wayland_client/struct.Filter.html
[`DispatchData`]: https://docs.rs/wayland-client/*/wayland_client/struct.DispatchData.html
