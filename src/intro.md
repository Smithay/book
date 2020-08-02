# Introduction

Welcome to the Smithay Handbook.

This is a work in progress, and intended to serve as a general manual for building Wayland-related
software in Rust, using the tools from the [Smithay project](https://smithay.github.io/).

The project revolves around 3 main components:

- The [wayland-rs](https://github.com/Smithay/wayland-rs) repository contains low-level bindings to the
  Wayland protocols, under the form of several crates. The two most notable being
  [`wayland-client`](https://crates.io/crates/wayland-client) and
  [`wayland-server`](https://crates.io/crates/wayland-server), which are the core bindings for client-side
  and server-side apps.
- [SCTK](https://crates.io/crates/smithay-client-toolkit), or Smithay Client ToolKit, is a crate designed
  to handle a significant portion of the plumbing required for writing Wayland client apps. It comes on top
  of wayland-client, and this book will present it as well.
- [Smithay](https://github.com/Smithay/smithay) is the flagship crate of the project, and is designed as
  a framework for writing Wayland servers (also called compositors). It is built on top of the wayland-server
  crate, and also handles most of the interaction with the system (input devices, graphics, udev, sessions, ...).

The first part of this book is dedicated to client-side apps, while the second part focuses of server-side.
If you are interested by the server-side stuff, it is recommended to first get familiar with the client-side,
as it is easier to get into and a lot of its concepts map to server-side.