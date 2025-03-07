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
- [ ] Via `ServerFn`
- [x] Direct (gated with `feature = "ssr"`)

A typical run:

```shell
$ cargo run --bin stress --features stress -- http://localhost:4000
  ...
     Running `target/debug/stress 'http://localhost:4000'`
ok    = 1000
err   = 0
sizes = {16952: 1000}
```

No errors produced, as no context nor resources were used, just direct
usage of signals.
