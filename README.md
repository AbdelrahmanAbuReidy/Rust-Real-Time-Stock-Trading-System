# Real-Time Stock Trading Simulation (Rust)

A two-process, real-time stock market simulation written in Rust. An **exchange service** owns a 60-symbol order book, streams market data every second and fluctuates prices on a random walk; a **broker service** consumes the feed, generates clients, validates orders against live bid/ask prices, and sends successful orders back — the two processes share no memory and communicate exclusively over RabbitMQ.

## Architecture

```mermaid
flowchart LR
    subgraph EX[stock binary - exchange service]
        INV["inventory\nArc&lt;Mutex&lt;Vec&lt;Stock&gt;&gt;&gt;\n60 symbols"]
        GEN[market publisher\nevery 1 s]
        FLU[price fluctuation\nevery 5 s, +/-5% walk]
        TRK[transaction tracker\nbought/sold per symbol]
        MON[monitoring threads\nCPU / memory / IO]
    end
    subgraph BR[trading binary - broker service]
        PARSE[feed parser]
        CLI[client generator\n1-5 clients per second]
        CHK[order validation\nmax_price vs bid/ask]
        PEND[pending-order backlog\nretry every 3 s]
    end
    GEN -- "queue: stocks\ntext feed, 1 s" --> MQ[(RabbitMQ)]
    MQ --> PARSE
    CHK -- "queue: order_queue\nJSON orders" --> MQ
    MQ --> INV
```

- **Exchange → broker:** the full order book is published each second to the `stocks` queue as a text feed; the broker re-parses it with explicit field-count and numeric validation.
- **Broker → exchange:** successful orders travel back over the `order_queue` queue as `serde_json` messages and mutate the authoritative inventory (`Buy` deducts, `Sell` adds), tracked per symbol for the end-of-run report.

## Key features

- **Two independent binaries** (`stock`, `trading`) coupled only by AMQP 0-9-1 via `amiquip` — a process can be restarted without taking the other down.
- **Market model:** 60 tech-sector symbols, bid/ask spread derived per tick (`buy_rate = 0.99 × price`, `sell_rate = 1.01 × price`), bounded ±5% uniform random-walk price fluctuation with a zero floor.
- **Multi-rate periodic tasks:** 1 s market publish, 1 s client arrivals, 3 s pending-order re-evaluation, 5 s price fluctuation, 1 s / 5 s monitoring — a bounded 90-second deterministic simulation window.
- **Thread-safe shared state:** the order book, AMQP channels, transaction tracker and broker are all shared across six concurrent threads via `Arc<Mutex<...>>`, with zero `unsafe` code.
- **Full order lifecycle with a retry backlog:** orders that fail affordability or inventory checks are parked in a pending queue and automatically re-evaluated against freshly fluctuated prices every 3 seconds until they clear.
- **Runtime instrumentation:** dedicated monitoring threads sample per-process and per-thread CPU utilisation (`cpu-monitor`, `cpu-time`), resident/virtual memory and process I/O counters (`perf_monitor`).
- **Dual message codec:** JSON (`serde`/`serde_json`) for order events plus a hand-written line parser for the market-data feed, with explicit validation on every field.

## Benchmarks (Criterion.rs)

Four micro-benchmarks (`harness = false`, `black_box`-guarded, I/O-free function variants) cover the hot paths:

| Bench | Measures |
| --- | --- |
| `generate_stock_vector_bench` | Construction cost of the full 60-symbol order book |
| `add_stock_bench` | Lock acquisition + linear symbol scan + quantity mutation (sell path) |
| `deduct_stock_bench` | Same under an inventory guard (buy path) |
| `fluctuate_price_bench` | One full market tick: 60 × (RNG + spread recalculation) |

```bash
cargo bench    # HTML reports in target/criterion/
```

## Running it

Requires RabbitMQ (default `amqp://guest:guest@localhost:5672`, overridable via `AMQP_URL`):

```bash
docker run -d --name rabbitmq -p 5672:5672 -p 15672:15672 rabbitmq:3-management

# terminal 1: exchange (market data + inventory)
cargo run --release --bin stock

# terminal 2: broker (clients + order flow)
cargo run --release --bin trading
```

Both processes self-terminate after the simulation window and the exchange prints a per-symbol bought/sold report.

## Project context

Developed for the Real-Time Systems module (BSc (Hons) Computer Science, Asia Pacific University) to exercise message-oriented architecture, thread-safe shared state and periodic task design in Rust.

## Author

**Abdelrahman Abu Reidy** — [github.com/AbdelrahmanAbuReidy](https://github.com/AbdelrahmanAbuReidy)
