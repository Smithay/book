# Smithay book conventions

To ensure the book is easy to read and understandable, you should follow the following conventions:

## Fully clarify crate/module resolution of types which are instantiated from any dependent libraries.

For example, the compositor section of the book uses `calloop`. When constructing an event loop, it is not
clear that the `EventLoop` is from calloop:

**Example:**
```rust
let mut event_loop = EventLoop::try_new().unwrap();
```

Clarify which crate the `EventLoop` originates from:
```diff
-let mut event_loop = EventLoop::try_new().unwrap();
+let mut event_loop = calloop::EventLoop::try_new().unwrap();
```
