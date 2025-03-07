# Issue 3671 demo.

The commit history for this example will show the variations between the
different combinations of context/resource/signal use.

## Quick Start

Run `cargo leptos watch` to first start the server.

Run `cargo run --bin stress --features stress -- http://localhost:4000`
in another terminal for stress testing the server to see the panics as
requests come in.

### Summary of issues using this app configuration

The configuration used:

Route `ssr=`:
- [x] `SsrMode::Async`
- [ ] Default (`SsrMode::OutOfOrder`)

Acquisition of `set_ctx` inside `on_cleanup`:
- [ ] `expect_context`
- [ ] `use_context`
- [x] Direct

Acquisition of `set_ctx` inside `hook`:
- [ ] `expect_context`
- [x] Direct

Method which `tokio::time::sleep` is invoked in the `hook`:
- [x] Via `ServerFn`
- [ ] Direct (gated with `feature = "ssr"`)

A typical run:

```shell
$ cargo run --bin stress --features stress -- http://localhost:4000
  ...
     Running `target/debug/stress 'http://localhost:4000'`
ok    = 960
err   = 40
sizes = {17088: 960}
```

Sticking with the direct signal usage, but reintroducing the use of
`Resource` + `ServerFn` back in, this different panic happens:

```
thread 'tokio-runtime-worker' panicked at /leptos/reactive_graph/src/owner/arena.rs:57:25:
at /leptos/reactive_graph/src/owner/arena.rs:60:29, the `sandboxed-arenas` feature is active, but no Arena is active
```

Again, ensure the cleanup only runs under `#[cfg(not(feature = "ssr"))]`
will prevent this panic.  This can be verified with `RUST_BACKTRACE=1`:

```
  16: issue_3671::app::__HomePage::{{closure}}
             at ./src/app.rs:54:24
```

The above line corresponds to the `set_ctx` call inside the `on_cleanup`
closure.

Moreover, only now I just realized I should check whether or not the
`on_cleanup` be called.  Changing that to log something, like so:

```rust
    on_cleanup(move || {
        leptos::logging::log!("on_cleanup");
        set_ctx.set(Ctx(None));
    });
```

Rerun the stress test using a single request will find the peculiar
behavior where the `on_cleanup` may not run, or run twice, or actually
trigger the error.
