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
- [x] `use_context`
- [ ] Direct

Acquisition of `set_ctx` inside `hook`:
- [x] `expect_context`
- [ ] Direct

Method which `tokio::time::sleep` is invoked in the `hook`:
- [x] Via `ServerFn`
- [ ] Direct (gated with `feature = "ssr"`)

A typical run:

```shell
$ cargo run --bin stress --features stress -- http://localhost:4000
  ...
     Running `target/debug/stress 'http://localhost:4000'`
ok    = 520
err   = 480
sizes = {17088: 520}
$ cargo ...
  ...
ok    = 521
err   = 479
sizes = {17088: 521}
```

The server process will produce the following panics:

100% is of the kind:

```
thread 'tokio-runtime-worker' panicked at /leptos/reactive_graph/src/owner.rs:258:26:
already mutably borrowed: BorrowError
```
