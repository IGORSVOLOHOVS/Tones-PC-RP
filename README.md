# Tones-PC-RP

The host-side protocol runner for the Tones instrument: it reads a protocol from
the database over HTTP, parses it into steps, and drives the hardware over a
serial port on a Raspberry Pi.

## Where this sits

Tones is one machine spread over six repositories - four STM32L432KC boards and
two host-side Rust programs. This is the program the operator actually starts;
the boards are firmware, and `TonesConnection` is the serial driver.

The whole instrument is described in the `Tones` manifest repository, including
the wiring, the command tables and one place where the host and the firmware
have drifted apart. Read `docs/architecture.md` there before changing anything
that goes over the wire.

```bash
pip install west
west init -l path/to/Tones
west update
```

## What is here

- `src/main.rs` - opens `/dev/ttyUSB0` at 9600 8N1 and runs the protocol loop.
  Steps are dispatched by prefix: `W_` wait, `TC_` temperature change,
  `BTC_` blocking temperature change, `LA_` liquid transfer between positions.
- `src/listen.py` - the listening side, in Python.
- `protocols.json` - protocol records as the database returns them.
- `diagrams/` - the use case, the send/receive sequence and the database layout.
- `qa/` - screenshots from the April and June 2024 QA passes.
- `TODO.md` - the original requirements, written as a project brief.

## Build

```bash
cargo build --release
```

Depends on `serialport`, `reqwest`, `serde` and `chrono`; see `Cargo.toml`.
