# Actix API

## Start api server

use `cargo run -p srv-api` or `cargo run -p actix-app-api api` to start the server

## Database commands

### migration

`cd crates/srv-storage && diesel migration run`
see the detail in [diesel.rs](https://diesel.rs/guides/getting-started/)

or

`cargo run -p actix-app-api db migration` to run database migration

---

support modules:

- [x] token list

  - [x] [polygon-token-list](https://github.com/maticnetwork/polygon-token-list) polygon
  - [x] [uniswap-token-list](https://github.com/Uniswap/token-lists)

- [x] 4bytes
  - [ ] [4bytes](https://www.4byte.directory/)
  - [x] [ethereum-lists/4bytes](https://github.com/ethereum-lists/4bytes)

project structure:

- [x] use clap
- [x] use clippy to build
- [x] health check
- [ ] prometheus metrics
- [x] tracing with jaeger
- [x] tracing with opentelemetry
- [x] tracing with opentelemetry and jaeger

---

## References

- [prometheus](https://prometheus.io/)
- [prom/prometheus docker](https://hub.docker.com/r/prom/prometheus)
- [jaegertracing](https://www.jaegertracing.io/)
- [jaegertracing docker](https://hub.docker.com/r/jaegertracing/all-in-one)
