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
- [x] `expect_context`
- [ ] `use_context`
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
ok    = 477
err   = 523
sizes = {17088: 477}
```

The server process will produce the following panics:

Roughly ~93% is of the kind:

```
thread 'tokio-runtime-worker' panicked at /leptos/reactive_graph/src/owner.rs:258:26:
already mutably borrowed: BorrowError
```

Roughly ~7% is of the kind:

```
thread 'tokio-runtime-worker' panicked at /leptos/reactive_graph/src/owner/context.rs:305:9:
Location { file: "src/app.rs", line: 55, col: 19 } expected context of type "reactive_graph::signal::write::WriteSignal<issue_3671::app::Ctx>" to be present
```
