# Introduction

Welcome to the Smithay Handbook.

This is a work in progress, and intended to serve as a general manual for building Wayland-related
software in Rust, using the tools from the [Smithay project](https://smithay.github.io/).

The [first section](./wayland/intro.md) is a general introduction on the Wayland protocol, its
goals and architecture.

The [second section](./client/intro.md) is dedicated to the building of client apps that can
run natively in any wayland environment, using [wayland-client](https://crates.io/crates/wayland-client)
and [SCTK](https://crates.io/crates/smithay-client-toolkit).

The [third section](./server/intro.md) focuses on building compositors (or wayland servers) that
act as both window managers and display servers, using [wayland-server](https://crates.io/crates/wayland-server)
and [Smithay](https://github.com/Smithay/smithay).

Have a good read!
