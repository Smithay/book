# The Wayland-rs &amp; Smithay book

Wayland &amp; Smithay book is an introductory book to using the [Wayland] display server protocol in the Rust programming language. This book is a guide for understanding the Wayland protocol and interfacing with the Wayland protocol using the [wayland-rs], [smithay-client-toolkit] and [smithay] crates.

You can read the book online [here](https://smithay.github.io/book/).

## Building the book

The wayland-rs &amp; Smithay book is built using [mdBook].

mdBook can be installed using cargo with the following:

```
$ cargo install mdbook
```

To build the book, run the following:

```
$ mdbook build
```

The output will be in the `book` subdirectory. The book can then be previewed in your web browser by opening the `index.html` file in the book subdirectory.

The book is also tested to ensure the source code in the book is correct. The following can be used to test if the source code referenced in the book compiles.

```
$ cargo check
```

[Rust]: https://www.rust-lang.org/
[Wayland]: https://wayland.freedesktop.org/
[mdBook]: https://github.com/rust-lang/mdBook

[wayland-rs]: https://github.com/Smithay/wayland-rs
[smithay-client-toolkit]: https://github.com/Smithay/client-toolkit
[smithay]: https://github.com/Smithay/smithay
