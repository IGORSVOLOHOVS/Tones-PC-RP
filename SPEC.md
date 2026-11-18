# Tones-PC-RP: software requirements specification

Reverse-engineered from commit `3a92e1e` on 2026-11-18. It describes what the
code at that commit does, not what the project brief or the README intend.
Those intentions are listed under "Declared but not implemented". The project
overview is in [`docs/README.md`](docs/README.md).

**Important:** `src/main.rs` does not compile at this commit. It uses an
undeclared crate (`serialport`, `src/main.rs:1`, absent from `Cargo.toml:8-14`),
an undefined variable (`command_str`, `src/main.rs:44`) and three undefined
functions (`src/main.rs:51,55,64`). This is inferred from the source. No Rust
toolchain was available to confirm it. FR-1 to FR-11 below therefore describe
the behaviour the source specifies, none of which has been observed running.

<!-- jira-sync:start -->
Jira: KAN-88

## Tasks

This list is one ticket in Jira project KAN, [KAN-88](https://igorsvolohovs.atlassian.net/browse/KAN-88). Each line is a checkbox there: tick it in Jira and the sync ticks it here; add a checkbox there and it is appended here. Done items are the features described in section 3; open items are what the project declares but the code does not do yet.

- [x] Serial link (section 3) <!-- spec-serial-link -->
- [x] Command dispatch (section 3) <!-- spec-command-dispatch -->
- [x] Step status reporting (section 3) <!-- spec-step-status-reporting -->
- [x] HTTP listener (section 3) <!-- spec-http-listener -->
- [ ] D-1 (declared, not implemented) <!-- spec-d-1-841273e6e5 -->
- [ ] D-2 (declared, not implemented) <!-- spec-d-2-05c019814e -->
- [ ] D-3 (declared, not implemented) <!-- spec-d-3-bce8c62d34 -->
- [ ] D-4 (declared, not implemented) <!-- spec-d-4-d8af4f1c27 -->
- [ ] D-5 (declared, not implemented) <!-- spec-d-5-d4570d50d5 -->
- [ ] D-6 (declared, not implemented) <!-- spec-d-6-08e3082394 -->
- [ ] D-7 (declared, not implemented) <!-- spec-d-7-cf2e6697bb -->
- [ ] D-8 (declared, not implemented) <!-- spec-d-8-e828a6e422 -->
- [ ] D-9 (declared, not implemented) <!-- spec-d-9-b02203f6ea -->

<!-- jira-sync:end -->

## 1. Purpose and scope

In scope:

- `src/main.rs`: a serial command loop for the host of the Tones instrument. It
  dispatches the text commands `W_`, `BTC_`, `TC_` and `LA_`, and writes step
  status strings back to the serial port.
- `src/listen.py`: an HTTP endpoint that accepts JSON status messages and prints them.
- `protocols.json`, `test.json`: sample protocol records.

Out of scope:

- The STM32L432KC board firmware, the `TonesConnection` serial driver and the
  `Tones` manifest repository, which `README.md:9-15` places in other repositories.
- The protocol database and the PC-side protocol editor shown in
  `diagrams/tones_sendreceive.png`.
- The lab tooling (`recipe.ps1`, `.claude/`, `.gitea/`), apart from NFR-6.

## 2. Context

| Actor / system | Role | Source |
|---|---|---|
| Developer | Target user named in the brief | `TODO.md:9` |
| Operator | Starts the program, according to the README | `README.md:10-11` |
| Serial peer on `/dev/ttyUSB0` | Receives status strings; meant to send commands | `src/main.rs:20,42,78` |
| HTTP client | Posts JSON to `listen.py` | `src/listen.py:5-7` |
| Protocol database | Holds protocols; not accessed by any code here | `diagrams/tones_databaseip.png`, `TODO.md:21-22` |

Runtime environment: Linux with a USB serial device at `/dev/ttyUSB0`
(`src/main.rs:20`). The README names a Raspberry Pi (`README.md:5`), and the brief
says "RP 4" (`TODO.md:7`). `listen.py` runs on any host with Python and Flask.

## 3. Functional requirements

### Serial link

- **FR-1.** The system shall open `/dev/ttyUSB0` at 9600 baud, 8 data bits,
  1 stop bit, no flow control and a 1000 ms timeout.
  Source: `src/main.rs:20-35`. Verification: no test.
- **FR-2.** The system shall terminate with the message `Failed to open serial port`
  when the port cannot be opened.
  Source: `src/main.rs:36`. Verification: no test.

### Command dispatch

- **FR-3.** The system shall process command strings in an endless loop and clear
  its read buffer after each command.
  Source: `src/main.rs:41-69`. Verification: no test.
- **FR-4.** The system shall treat a command starting with `W_` as a wait: it parses
  the rest as `u64` milliseconds and calls `waiting_command` with the step name
  `Waiting Command`.
  Source: `src/main.rs:45-48`. Verification: no test.
- **FR-5.** The system shall treat a command starting with `BTC_` as a blocking
  temperature change: it parses the rest as `f32` and uses the step name
  `Blocking Temp Change`.
  Source: `src/main.rs:49-52`. Verification: no test. The handler is not defined (D-5).
- **FR-6.** The system shall treat a command starting with `TC_` as a
  non-blocking temperature change: it parses the rest as `f32` and uses the step
  name `Non-Blocking Temp Change`.
  Source: `src/main.rs:53-56`. Verification: no test. The handler is not defined (D-5).
- **FR-7.** The system shall treat a command `LA_<from>_<to>_<volume_ml>` as a
  liquid transfer: it parses the three fields as `u32` and uses the step name
  `Liquid Application`.
  Source: `src/main.rs:57-65`. Verification: no test. The handler is not defined (D-5).
- **FR-8.** The system shall print `Unknown command: <command>` to standard output
  for any other command.
  Source: `src/main.rs:66`. Verification: no test.

### Step status reporting

- **FR-9.** For each dispatched command, the system shall create a status
  generator that, as it is resumed, yields `<step> - Processing`, waits 2 s, and
  then yields `<step> - OK`, or `<step> - Error` when the step name is `Step 3`.
  Only `waiting_command` resumes it; for the other three commands that is up to
  the undefined handlers (D-5).
  Source: `src/main.rs:8-17,46,50,54,63`. Verification: no test.
- **FR-10.** For a wait command, the system shall write each status string to
  the serial port. It stops after the first status that contains `Error` or `OK`,
  or when the status sequence ends.
  Source: `src/main.rs:76-87`. Verification: no test.
- **FR-11.** The system shall report a failed serial write to standard error as
  `Error writing to serial port: <error>` and continue.
  Source: `src/main.rs:78-80`. Verification: no test.

### HTTP listener

- **FR-12.** The listener shall accept `POST /` with a JSON body and print
  `Received POST data: <body>`.
  Source: `src/listen.py:5-8`. Verification: no test.
  (`qa/GET_request_python.png` shows a manual run on 2024-05-01.)
- **FR-13.** The listener shall answer every such request with HTTP 200 and the
  body `POST request received!`.
  Source: `src/listen.py:9`. Verification: no test.
- **FR-14.** When started as a script, the listener shall run the Flask
  development server with debug mode on.
  Source: `src/listen.py:11-12`. Verification: no test.

### Declared but not implemented

| Id | Claim | Where claimed | State in code |
|---|---|---|---|
| D-1 | Read a protocol from the database over HTTP | `README.md:3-4`, `recipe.ps1:12`, `TODO.md:24-25` | No HTTP client call. `reqwest` (`Cargo.toml:14`) is unused |
| D-2 | Parse the protocol into steps | `README.md:4`, `TODO.md:26` | No JSON parsing. `serde*` and `chrono` (`Cargo.toml:10-13`) are unused, and nothing reads `test.json` or `protocols.json`. A 2024-05-01 version did parse `test.json` (`git show e5b4da1:src/main.rs`) |
| D-3 | Read commands from the serial port | `src/main.rs:42` (placeholder comment) | `reader` and `buffer` (`src/main.rs:38-39`) are never read from |
| D-4 | Wait for the given time | `src/main.rs:75` (placeholder comment) | `time_ms` is unused |
| D-5 | Blocking and non-blocking temperature change, liquid application | `src/main.rs:90` (placeholder comment) | The functions are not defined |
| D-6 | Database access through Prisma (`schema.prisma`, `*.sql`) | `TODO.md:5,21-24` | No Prisma or SQL files or code |
| D-7 | Error correction and a logger | `diagrams/use-case.png` ("Future: error correction and logger") | None |
| D-8 | PC "Protocols Modifier" and "Protocols ID sender"; RP "Protocols ID receiver" and "Protocols getter" | `diagrams/tones_sendreceive.png` | None |
| D-9 | C/C++ stack, GTest, benchmarks, Docker, CI/CD | `TODO.md:50-91` | None. The checked "CMake" item (`TODO.md:84`) has no CMake file |

## 4. Interfaces

Serial (`src/main.rs`):

| Item | Value | Source |
|---|---|---|
| Device | `/dev/ttyUSB0` | `src/main.rs:20` |
| Line settings | 9600 baud, 8 data bits, 1 stop bit, no flow control; parity not set | `src/main.rs:21-24` |
| Inbound | `W_<u64>`, `BTC_<f32>`, `TC_<f32>`, `LA_<u32>_<u32>_<u32>` | `src/main.rs:45-65` |
| Outbound | `<step> - Processing`, `<step> - OK`, `<step> - Error`, with no terminator | `src/main.rs:10-14,78` |

HTTP (`src/listen.py`):

| Method | Path | Request | Response |
|---|---|---|---|
| POST | `/` | JSON body, any shape | 200, text `POST request received!` |

In the 2024-05-01 version, the body that `main.rs` posted was
`{"protocol_id": int, "step_id": int, "status": "OK" or an error text}`
(`git show e5b4da1:src/main.rs`). No code at this commit produces it.

## 5. Data

There is no runtime storage. The code writes no files and has no database.
The sample files:

| File | Shape |
|---|---|
| `protocols.json` | Array of `{id: int, author: string, creationDate: ISO 8601 string, description: string, name: string}` |
| `test.json` | Array of protocols: `{id, name, customLiquids: [], description, steps: [Step], creationDate, defaultWash: {iters, incubation, liquid: Liquid, temperature: null}, author: null}` |

`Step` is `{type, id, params}`. The `params` depend on `type` (`test.json`):

| `type` | `params` |
|---|---|
| `Liquid Application` | `autoWash: bool`, `liquid: Liquid`, `incubation: int`, `temperature: int` |
| `Washing` | `liquid: Liquid`, `iters: int`, `incubation: int`, `temperature: int` |
| `Temperature Change` | `source: int`, `target: int` |

`Liquid` is `{type: {id, name}, id, name, usedCold: bool, toxic: bool}`. Units of
`incubation` and `temperature` are not stated anywhere.

## 6. Non-functional requirements

- **NFR-1.** Platform: the program targets Linux serial device naming
  (`/dev/ttyUSB0`). Source: `src/main.rs:20`.
- **NFR-2.** Responsiveness: a serial read times out after 1000 ms, and each
  simulated step lasts 2 s. Source: `src/main.rs:34`, `src/main.rs:11`.
- **NFR-3.** Input validation, as implemented: none. A malformed numeric field,
  or an `LA_` command with fewer than three fields, ends the process with a panic.
  Source: `src/main.rs:47,51,55,58-61`.
- **NFR-4.** Language level: Rust edition 2021. Source: `Cargo.toml:4`.
- **NFR-5.** Exposure: the listener uses Flask's default bind address, which
  `qa/GET_request_python.png` shows as `127.0.0.1:5000`. Its debugger is on.
  Source: `src/listen.py:12`.
- **NFR-6.** The repository follows the repo-quality-template lab tooling against
  lab requirements revision 4.0.0, with the listed exceptions. `.\recipe.ps1 test`
  checks only `.claude/`. Source: `README.md:59-61`, `recipe.ps1:41-55`.

## 7. Constraints and dependencies

| Dependency | Declared (`Cargo.toml`) | Locked (`Cargo.lock`) | Used at this commit |
|---|---|---|---|
| `generator` | `0.7` (line 9) | not locked | yes (`src/main.rs:4`) |
| `serde` (derive) | `1.0.199` (line 10) | 1.0.199 | no |
| `serde_json` | `1.0.116` (line 11) | 1.0.116 | no |
| `chrono` | `0.4` (line 12) | 0.4.38 | no |
| `serde_with` | `3.8.1` (line 13) | 3.8.1 | no |
| `reqwest` (blocking) | `0.12.4` (line 14) | 0.12.4 | no |
| `serialport` | not declared | not locked | yes (`src/main.rs:1`) |
| Flask (Python) | no requirements file | - | yes (`src/listen.py:1`) |

No Rust toolchain version and no Python version are pinned. There is no licence
file, deliberately (`README.md`, "Licence").

## 8. Acceptance

1. `cargo build --release` (`README.md:40`) succeeds. It is expected to fail at
   this commit; this is a precondition for checking FR-1 to FR-11.
2. With a serial peer on `/dev/ttyUSB0` and D-3 implemented, the command
   `W_500` makes the peer receive `Waiting Command - Processing` and, about 2 s
   later, `Waiting Command - OK` (FR-4, FR-9, FR-10).
3. A command with an unknown prefix prints `Unknown command: <command>` (FR-8).
4. With `src/listen.py` running, a JSON `POST /` gets HTTP 200 with
   `POST request received!`, and the body appears on the console (FR-12, FR-13).
5. `.\recipe.ps1 test` exits 0 (NFR-6).

No automated test covers any of these.

## 9. Open questions

- Where do command strings come from? Reading is a placeholder
  (`src/main.rs:42`). Which device sits on `/dev/ttyUSB0`, given four
  STM32L432KC boards (`README.md:9`)?
- How are protocol steps (`test.json`) meant to become serial commands? There is
  no serial command for `Washing`, and `W_` has no JSON step type. A TypeScript
  `CommandType` enum in `qa/Screenshot from 2024-06-19 07-18-37.png`, whose
  source is not here, does list `Waiting` beside `LiquidApplication` and
  `TemperatureChange`, and has no washing either.
- Parity: `README.md:26` says 8N1, but the code sets no parity
  (`src/main.rs:27-35`).
- What terminator or framing does the serial peer expect for status strings?
- Where is the protocol database, at what address and with what schema? The
  `ip:port` questions in `qa/tones-qa-27-04-2024.png` are unanswered, and
  `schema.prisma` (`TODO.md:21`) is not in the repository.
- Which Raspberry Pi model is the target? "RP 4" (`TODO.md:7`) is read here as
  Raspberry Pi 4, which is an inference.
- What are the units of temperature, `incubation` and volume? Only `volume_ml`
  names a unit (`src/main.rs:61`).
- Is `listen.py` meant for anything beyond local testing? Nothing in the
  repository calls it at this commit; its only known caller is the 2024-05-01
  `main.rs` (section 4).
- `TODO.md:19` states, in transliterated Russian, that the device must work
  without endangering the chemist. No safety requirement is written down or
  implemented.
- Should the crate version `Cargo.toml:3` (0.1.0) follow the repository version
  `recipe.ps1:11` (1.0.0)? Releases use the latter: the release workflow tags
  `v<$Version>` from `recipe.ps1` (`.gitea/workflows/release.yaml:45-51`), tag
  `v1.0.0` points at `3a92e1e`, and nothing reads `Cargo.toml`'s version.
