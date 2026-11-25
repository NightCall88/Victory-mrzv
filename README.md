# Victory-mrzv — a VIctory fork for one NVIDIA PC (work in progress)

A work-in-progress fork of [VIctory](https://github.com/Marvisak/VIctory) —
through [fxceless/victory-fork](https://github.com/fxceless/victory-fork) — a
compatibility layer that runs an Xbox Series title natively on Windows from
files the user supplies. This fork follows that line and adds fixes aimed at one
goal: getting the game's world to render correctly, and as fast as a translated
frame can be, on one particular PC.

**This repository contains no game files, code or assets.** You need your own
copy of the game. It is not affiliated with or endorsed by the game's developer
or publisher.

## Credits

- **[VIctory](https://github.com/Marvisak/VIctory)** by Marvisak and its
  contributors is the base of everything here: the loader, the runtime services,
  the Direct3D 12 back end and the shader recompiler. Licensed GPL-3.0-or-later;
  so is this fork. Upstream's history is kept intact below the commits that
  carry this fork's changes.
- **[fxceless/victory-fork](https://github.com/fxceless/victory-fork)** is the
  line this fork starts from: the descriptor fingerprint, the first room, the
  submit thread and the copy queue are its work, carried in the history. Its own
  status section describes that fork, not this one.
- **[Belobug/VIctory-Belobug-AMD](https://github.com/Belobug/VIctory-Belobug-AMD)**
  and **[uiliannn/VItoriaBr](https://github.com/uiliannn/VItoriaBr)** were read
  for their measurements — the big-buffer pair and the fetching switch above all
  — and not copied from.
- **[ubiquitarianism/leonida](https://github.com/ubiquitarianism/leonida)**
  documented the console's texture layouts and the reading of `layout 0x101`.
- **[xenia-canary](https://github.com/xenia-canary/xenia-canary)** is where the
  shape of the submission pipeline came from: a frame waits for the frame three
  before it, so the wait is empty.
- **[AnyPS5](https://github.com/boykopovar/AnyPS5)** shows the exact half-wave
  exchange this fork currently approximates.
- Developed with **DeepSeek** as the coding model, and **Claude** for the
  analysis that produced the measurements in
  [docs/MEASUREMENTS.md](docs/MEASUREMENTS.md).

## Status of this fork (2026-11-26)

**Playable in the prologue. Not yet a port you would play through: 5 to 8 frames
per second at 1920x1080, and the physics runs at the host's frame time.** Tested
on one machine: Windows 10 22H2 x64, NVIDIA RTX 4070 SUPER 12 GB, 32 GB RAM. The
Series X profile is used throughout.

| Stage | State in this fork |
|---|---|
| Boot, menus, start screen | works, 30 fps (as upstream) |
| Opening cutscene | plays from start to finish, in order, lit |
| Saves | the game autosaves a checkpoint and loads it again |
| The world: prologue, prison yard, beach, water, cutscenes | renders — lit, textured, with people, vehicles, vegetation, water and sky |
| Frame rate in the world | 5–8 fps: 120–183 ms a frame on a beach scene of 1 400–1 700 draws; 3–4 fps and 200–600 ms in the heaviest cutscenes |
| Light in shadow | a constant stands in for it: the game's ray-traced bounce light and its ray-traced lamp and sun shadows do not run, so anything the sun does not hit directly is dark |
| Water | steady in the picture; the simulation that feeds it is one of four compute programs that do not translate |
| Boats | sit low and sink under a static water surface, and the player drowns: the fluid simulation's compute programs are dropped (upstream issue #2248 names the same fault) |
| Distant trees and the far imposters | not drawn |
| Stability | no driver hang or device loss in normal runs of this fork |

### What this fork changed

Picture:

- **The 16-bit full-screen targets keep their channel order.** The console's
  `B5G5R5A1` (format 86) and `B4G4R4A4` (115) are blue-first; their stand-in was
  `R8G8B8A8` (28), which is red-first, so blue landed in red on every such
  target. Measured on a capture: clipped red 34.4 %, clipped blue 14.2 %, and
  the picture read as a red cast. With `B8G8R8A8` (87) the three channels clip
  equally, 2.7 % each.
- **A compute program that needs 64-lane waves runs in waves of 32**
  (`VICTORY_WAVE_FORCE32=1`). A host with 32-lane waves refuses `WaveSize(64)`
  outright and a program with no pipeline draws nothing at all: seven of the
  title's compute programs — the screen-tile pass among them, which the
  reflection and lighting passes read — had no pipeline, and the water
  flickered. Split in halves by group barriers, with the halves exchanging
  through two dwords past the program's LDS, they build. One refusal remains,
  the mesh pipeline below. This is an approximation of the console's LDS
  sharing; the exact form is what AnyPS5 and Leonida do in SPIR-V.
- **The window the title asks for can be sized** (`VICTORY_WINDOW_SIZE=WxH`). A
  console title sizes its window to its render target — 3840x2160 — and the swap
  chain and the title's own targets follow it, so the world was drawn at 4K. On
  a host whose screen is that big nothing was fitted and the picture stayed 4K.
  At 1920x1080 the same scene draws in 50–90 ms of GPU time instead of 110–120 ms.

Speed and memory:

- **Copies of what shaders wrote go back through a plain move**
  (`VICTORY_MEMCPY_WRITES`, on). Writing into memory this process owns went
  through `NtWriteVirtualMemory` a range at a time; the sampled profile put it at
  49 ms a frame of kernel time, 44 MB/s.
- **A copy flushes only the copies it can have made stale.** Every one of a
  submission's ~20 000 recorded copies flushed every pending refresh, so the same
  945 MB was re-checked about a dozen times a frame: 29 678 refreshes became
  18 511.
- **More of the frame is in the air before it is settled**
  (`VICTORY_LATE_WAIT_LISTS=1024`, 256 before). The submitting thread used to
  stand in `WaitForSingleObject` for 111 ms of a 137 ms settle, `on cpu` 3 %: the
  CPU and the card ran in series. With a deeper queue the frame holds 5–7 fps on
  scenes that fell to 3.
- **Submissions are prepared and sent by a thread of their own**
  (`VICTORY_ASYNC_SUBMIT=1`), and fetches of what shaders wrote go out on a copy
  queue (`VICTORY_QUEUE2=1`): copy-back 430 ms → 58 ms → 4.9 ms on its own queue.
- **The big title buffers stay in the card's memory with a 1 GB budget**
  (`VICTORY_CARD_BUFFERS_MIN_MB=1`, `VICTORY_CARD_BUFFERS_MB=1024`), the pair
  measured by ztrx on the same card: 420–450 ms → 240–350 ms.

Platform:

- **The Agility SDK is exported from the executable.** Windows 10's in-box
  `d3d12.dll` serves shader model 6.5 and refuses directly-indexed descriptor
  heaps, which this title uses. `D3D12SDKVersion` and `D3D12SDKPath` are exported
  as data symbols — the second a pointer to an ASCII path, not a buffer — with
  `D3D12Core.dll` beside the executable.

Most of the newer switches are off by default and each is described in a comment
where it is read. The set this fork was measured with is in
[`run/run-1080.ps1`](run/run-1080.ps1), with the measurement behind every line.

### Known problems

- Everything in shadow is dark; the title's ray-traced light does not run (see
  above).
- **Boats sink.** The title measures its frame with `QueryPerformanceCounter`
  and integrates its physics over that interval; at 6 fps gravity acts for
  160 ms a tick while buoyancy answers once. Advancing the title's clock by
  frames instead (`VICTORY_GUEST_CLOCK_MS`) was built and **crashed the title**
  (`0xc0000005`, write to 0): it reads that counter consistently with clocks
  this layer does not own. The switch is there, off.
- **The mesh pipeline** `mesh f5e50eb2108c425b` with `pixel ce3f0432a08f1d35` is
  refused: its mesh outputs exceed the 32 KiB a mesh program may write, the
  pixel program is linked against the full set of signatures, and D3D12 refuses
  the pair.
- **Four compute programs do not translate**, all for the same reason — a wave
  reduction, a first-lane read or a lane-mask exchange where not every lane runs.
  The tile and fluid passes are among them.
- Around 20 GB of memory is in use during play.
- Many changes here are specific to this title and this GPU, and some are marked
  `shortcut:` in the code where they approximate what the console does.

The frame line this build prints carries its own breakdown; every number in
[docs/MEASUREMENTS.md](docs/MEASUREMENTS.md) was read off it or off a sampled
profile of it.

Everything upstream of this fork — VIctory's own status, its subsystem table and
its debugging aids — is unchanged and linked from
[docs/upstream-status.md](docs/upstream-status.md).

## Getting started

Requirements: Windows 10/11 x64, a CPU with AVX2, a Direct3D 12 GPU with shader
model 6.6 and mesh shaders, 32 GB of RAM, Rust (pinned in
`rust-toolchain.toml`).

```
git clone https://github.com/NightCall88/Victory-mrzv
cd Victory-mrzv
copy victory.example.toml victory.toml      # then set your paths (outside the repo)
cargo build --release -p victory --features diagnostics
```

`D3D12Core.dll` (Agility SDK 620) goes in a `D3D12` folder beside the executable.

```
target\release\victory.exe run <your game dir>\<the title's exe> --execute --console series-x
```

The window caption shows the framerate. Saves, settings and `crashes.log` are
kept under the data folder (default `%LOCALAPPDATA%\victory\data`), derived data
under the cache folder.

Input: XInput pads. With none connected the keyboard is a pad while the title's
window has focus:

| Keys | Pad |
|---|---|
| W A S D / I J K L | left / right stick |
| arrows | D-pad |
| Enter, Backspace, X, Y | A, B, X, Y |
| Q / E, Z / C | shoulders, triggers |
| F / H | stick clicks |
| Esc, Tab | Menu, View |

## Commands

```
victory inspect <exe|dll> [--imports] [--exports]
victory census [paths...] [--missing] [--strict]
victory lib-exports <import.lib> [--list]
victory shader-info [paths...] [--verbose] [--disasm]
victory translate [paths...] [--validate] [--pipelines] [--list] [--jobs N]
victory run <exe> [--execute] [--unimplemented trap|zero] [--strict]
            [--console series-x|series-s] [-- <title arguments>]
victory config
```

Logging: `VICTORY_LOG=victory_loader=trace,info` (a release build logs info and
above).

## Debugging

A release build logs info and above. The aids below are compiled in by
`cargo build --release -p victory --features diagnostics`.

| Variable | Effect |
|---|---|
| `VICTORY_LOG=<target>=debug,...` | debug and trace output |
| `VICTORY_PROFILE=1` | a sampled profile of the submitting, recording and worker threads, written every 30 s and at exit |
| `VICTORY_COPY_LINES=1` | the copies behind the frame line: title copies, refreshes, what the write watch found changed |
| `VICTORY_WAVE_WHY=1` | why a compute program keeps 64-lane waves, one line per reason |
| `VICTORY_WAVE_FORCE32=1` | run such a program in waves of 32, its LDS split by group barriers |
| `VICTORY_WINDOW_SIZE=WxH` | the size the title's window is made |
| `VICTORY_MEMCPY_WRITES=0` | go back to `NtWriteVirtualMemory` for write-backs |
| `VICTORY_LATE_WAIT_LISTS=<n>` | how many host lists may be in the air before the flights are settled |
| `VICTORY_BVH_BUILDER=1` | try the ray-tracing structure builder (off: not all of its programs have a host pipeline) |
| `VICTORY_D3D12_DEBUG=1` | the host debug layer, when Windows' Graphics Tools is installed |
| `VICTORY_D3D12_WARP=1` | host device on the software adapter (slow; cannot reset the driver) |
| `VICTORY_CAPTURE=<dir>`, `VICTORY_DUMP_TARGETS=1` | presented frames and every render target of them |

The full list is what the sources read — 206 switches; the ones that matter are
in the run script, each with the measurement behind it.

## License

GPL-3.0-or-later. See [LICENSE](LICENSE).

## Files

[.cargo/](.cargo), [.githooks/](.githooks), [apps/victory/](apps/victory),
[crates/](crates), [docs/](docs), [run/](run),
[tests/fixtures/hle-probe/](tests/fixtures/hle-probe), [xtask/](xtask),
[.gitattributes](.gitattributes), [.gitignore](.gitignore),
[.victory-root](.victory-root), [Cargo.lock](Cargo.lock), [Cargo.toml](Cargo.toml),
[LICENSE](LICENSE), [README.md](README.md),
[rust-toolchain.toml](rust-toolchain.toml), [rustfmt.toml](rustfmt.toml),
[victory.example.toml](victory.example.toml)