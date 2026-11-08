# tones-pc-rp

The host-side protocol runner for the Tones instrument: it reads a protocol from
the database over HTTP, parses it into steps, and drives the hardware over a
serial port on a Raspberry Pi.

## Where this sits

Tones is one machine spread over six repositories: four STM32L432KC boards and
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

- `src/main.rs` opens `/dev/ttyUSB0` at 9600 8N1 and runs the protocol loop.
  Steps are dispatched by prefix: `W_` wait, `TC_` temperature change,
  `BTC_` blocking temperature change, `LA_` liquid transfer between positions.
- `src/listen.py` is the listening side, in Python.
- `protocols.json` and `test.json` hold protocol records as the database returns them.
- `diagrams/` holds the use case, the send/receive sequence and the database layout.
- `qa/` holds screenshots from the April and June 2024 QA passes.
- `TODO.md` holds the original requirements, written as a project brief, partly in Russian.

## Build

On the Raspberry Pi, or any machine with a Rust toolchain:

```bash
cargo build --release
```

It depends on `serde`, `serde_json`, `serde_with`, `reqwest`, `chrono` and
`generator`; see `Cargo.toml`.

## Working on it

This repository ships no program from this Gitea. `.\recipe.ps1` is still its
whole interface: `.\recipe.ps1 test` runs pytest and ruff over `.claude`, and
nothing over `src/`, because Rust is not installed on the machine that runs the
gates. `.\recipe.ps1 info` prints the metadata. The other verbs print
`nothing to <verb>`. `CLAUDE.md` holds the rules.

## Licence

There is none, deliberately. Ownership of the Tones work is not settled, so the
absence of a licence file means default copyright, all rights reserved, until it is.

## Lab requirements

This repository complies with [revision 4.0.0](http://127.0.0.1:3000/igors-lab/software-requirements/src/tag/v4.0.0/BEST_REQUIREMENTS.md) of the lab requirements, except R-02 (#7), R-09 (#8), R-13 (#9), R-18 (#10), R-37 (#11), R-41 (#12), R-42 (#13).
