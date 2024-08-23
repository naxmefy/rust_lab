## Nightly Toolchain

> [!IMPORTANT]  
> As explained in the leptos book (here https://book.leptos.dev/getting_started/index.html)
>
> Using `nightly` Rust, and the `nightly` feature in Leptos enables the function-call syntax for signal getters and setters that is used in most of this book.
>
> To use nightly Rust, you can either opt into nightly for all your Rust projects by running
>
> ```bash
> rustup toolchain install nightly
> rustup default nightly
> ```
>
> or only for this project
>
> ```bash
> rustup toolchain install nightly
> cd <into your project>
> rustup override set nightly
> ```
>
> [See here for more details.](https://doc.rust-lang.org/book/appendix-07-nightly-rust.html)
>
> If you’d rather use stable Rust with Leptos, you can do that too. In the guide and examples, you’ll just use the [`ReadSignal::get()`](https://docs.rs/leptos/latest/leptos/struct.ReadSignal.html#impl-SignalGet%3CT%3E-for-ReadSignal%3CT%3E) and [`WriteSignal::set()`](https://docs.rs/leptos/latest/leptos/struct.WriteSignal.html#impl-SignalGet%3CT%3E-for-ReadSignal%3CT%3E) methods instead of calling signal getters and setters as functions.
