# Actix API

use `cargo run -p srv-api` to start the server

---

support modules:

- [x] token list

  - [x] [polygon-token-list](https://github.com/maticnetwork/polygon-token-list) polygon
  - [x] [uniswap-token-list](https://github.com/Uniswap/token-lists)

- [ ] 4bytes
  - [ ] [4bytes](https://www.4byte.directory/)
  - [ ] [ethereum-lists/4bytes](https://github.com/ethereum-lists/4bytes)

project structure:

- [] use clap
- [x] use clippy to build
- [ ] health check
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
