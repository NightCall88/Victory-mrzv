# VIctory — a Windows port of GTA VI's Xbox Series build, run from your own copy

Runs *Grand Theft Auto VI* (Xbox Series X|S, 1.0.2.1) on Windows from files the
user supplies. The title's own x86-64 code runs unchanged inside our process;
this layer supplies the platform underneath it and converts the title's
D3D12.X calls (the console's D3D12 dialect) into host D3D12, translating the
console's RDNA2 GPU programs to DXIL on first use.

This is a fork of [fxceless/victory-fork](https://github.com/fxceless/victory-fork),
which is a fork of [Marvisak/VIctory](https://github.com/Marvisak/VIctory). It
carries that history plus the patches described below, every one of them made
while running the world and measured on the machine listed under *Tested on*.

**The repository contains no game files, no keys and no executable.** You need
your own copy of the game, with its files readable on disk. Nothing here
decrypts or works around any protection.

## Status (2026-11-26)

**Playable in the prologue, not yet a port you would play through.** The world
renders — lit, textured, with people, vehicles, vegetation, water and sky — at
5 to 8 frames a second on a 1920x1080 picture. Missions start, saves load and
store, pads and keyboards drive it.

| Stage | State |
|---|---|
| Boot, first-boot screens, intro video with sound | works |
| Menus, profile, saves | works, 30 fps |
| Loading into the world (about 50 s) | works |
| The world: prologue, prison yard, beach, cutscenes | renders and runs |
| Frame rate in the world | 5–8 fps; 120–180 ms a frame; 3–4 fps in the heaviest scenes |
| Ray-traced light | does not run; shaded places carry a constant instead |
| Boats | sit low and sink under a static water surface (the title's physics runs at the host's frame time) |
| Distant trees, the far imposters | not drawn |

Measured, not estimated: the frame line this build prints carries its own
breakdown, and the numbers in [docs/MEASUREMENTS.md](docs/MEASUREMENTS.md) were
read off it.

> **Warning.** A host GPU failure can reset the display driver (every monitor
> goes black for a moment). Every build appends such failures to `crashes.log`
> in the data folder.

## What this fork changes over its parents

**Picture**

- **The 16-bit full-screen targets keep their channel order.** The console's
  `B5G5R5A1` (format 86) and `B4G4R4A4` (115) are blue-first; their stand-in
  was `R8G8B8A8` (28), which is red-first, so blue landed in red on every such
  target and the picture read as a red cast with red clipped in twice as many
  pixels as blue. Measured on a capture: clipped red 34.4 %, clipped blue
  14.2 %. With `B8G8R8A8` (87) as the stand-in the three channels clip equally,
  2.7 % each. See `crates/victory-gpu/src/resource.rs`.
- **A compute program that needs 64-lane waves runs in waves of 32**
  (`VICTORY_WAVE_FORCE32=1`). A host whose waves are at most 32 lanes refuses
  `WaveSize(64)` outright, and a program with no pipeline draws nothing at
  all: seven of the title's compute programs — the screen-tile pass among them,
  which the reflection and lighting passes read — had no pipeline and the water
  flickered. With the two halves split by group barriers and the halves
  exchanging through two dwords past the program's LDS they build, and the
  water is steady. It is an approximation of the console's LDS sharing; the
  exact form is what [AnyPS5](https://github.com/boykopovar/AnyPS5) and
  [Leonida](https://github.com/ubiquitarianism/leonida) do differently, in
  SPIR-V.

**Speed**

- **The window the title asks for can be sized** (`VICTORY_WINDOW_SIZE=WxH`).
  A console title sizes its window to its render target — 3840x2160 — and the
  swap chain and the title's own targets follow it, so the world was drawn at
  4K. On a host whose screen is that big nothing was fitted and the picture
  stayed 4K. At 1920x1080 the same scene draws in 50–90 ms of GPU time instead
  of 110–120 ms.
- **Copies of the card's own memory go through a plain move**
  (`VICTORY_MEMCPY_WRITES`, on). Writing what shaders wrote back into our own
  memory went through `NtWriteVirtualMemory` a range at a time: the sampler put
  it at 49 ms a frame of kernel time, 44 MB/s, for memory this process already
  owns and whose guard pages the caller had opened.
- **A copy flushes only the copies it can have made stale.** Every one of a
  submission's ~20 000 recorded copies flushed every pending refresh, so the
  same 945 MB was re-checked about a dozen times a frame: 29 678 refreshes
  became 18 511.
- **More of the frame is in the air before it is settled**
  (`VICTORY_LATE_WAIT_LISTS=1024`, 256 before). The submitting thread used to
  stand in `WaitForSingleObject` for 111 ms a frame of a 137 ms settle, `on cpu`
  3 %: the CPU and the card ran in series. With a deeper queue the frame keeps
  5–7 fps on scenes that used to fall to 3.
- **Submissions are prepared and sent by a thread of their own**
  (`VICTORY_ASYNC_SUBMIT=1`) and fetches of what shaders wrote go out on a
  copy queue (`VICTORY_QUEUE2=1`): copy-back 430 ms → 58 ms, then to 5 ms on
  its own queue.

**Platform**

- **The Agility SDK is exported from the executable.** Windows 10's in-box
  `d3d12.dll` serves shader model 6.5 and refuses directly-indexed descriptor
  heaps, which this title uses. `D3D12SDKVersion` and `D3D12SDKPath` are
  exported as data symbols (the second a pointer to an ASCII path, not a
  buffer) with `D3D12Core.dll` beside the executable: 6.6, and the heaps build.

## Getting started

Requirements: Windows 10/11 x64, a Direct3D 12 GPU with shader model 6.6 and
mesh shaders, 32 GB of RAM, Rust (pinned in `rust-toolchain.toml`).

```
git clone --recursive <this repository>
cd victory
copy victory.example.toml victory.toml      # then set your paths (outside the repo)
cargo build --release -p victory --features diagnostics
```

`D3D12Core.dll` (Agility SDK 620) goes in a `D3D12` folder beside the
executable. Then:

```
target\release\victory.exe run <your game dir>\<the title's exe> --execute --console series-x
```

The settings this fork was measured with are in
[`run/run-1080.ps1`](run/run-1080.ps1): every `VICTORY_*` switch with the value
it was measured at, and a comment per switch saying what it cost before.

## Commands

```
victory inspect <exe|dll> [--imports] [--exports]
victory census [paths...] [--missing] [--strict]
victory shader-info [paths...] [--verbose] [--disasm]
victory translate [paths...] [--validate] [--pipelines] [--list] [--jobs N]
victory run <exe> [--execute] [--unimplemented trap|zero] [--strict]
            [--console series-x|series-s] [-- <title arguments>]
victory config
```

## Debugging

A release build logs info and above. The aids below are compiled in by
`cargo build --release -p victory --features diagnostics`.

| Variable | Effect |
|---|---|
| `VICTORY_LOG=<target>=debug,...` | debug and trace output |
| `VICTORY_PROFILE=1` | a sampled profile of the submitting, recording and worker threads, written every 30 s and at exit |
| `VICTORY_COPY_LINES=1` | the copies behind the frame line: title copies, refreshes, what the write watch found changed |
| `VICTORY_WAVE_WHY=1` | why a compute program keeps 64-lane waves, one line per reason |
| `VICTORY_BVH_BUILDER=1` | try the ray-tracing structure builder (off by default: not all of its programs have a host pipeline) |
| `VICTORY_D3D12_DEBUG=1` | the host debug layer, when Windows' Graphics Tools is installed |
| `VICTORY_D3D12_WARP=1` | host device on the software adapter (slow; cannot reset the driver) |
| `VICTORY_CAPTURE=<dir>`, `VICTORY_DUMP_TARGETS=1` | presented frames and every render target of them |

The full list — 206 switches — is what the sources read; the ones that matter
are in the run script, each with the measurement behind it.

## Credits

- **[Marvisak/VIctory](https://github.com/Marvisak/VIctory)** by Marvisak and
  its contributors is the base of everything here. Licensed
  GPL-3.0-or-later; so is this.
- **[fxceless/victory-fork](https://github.com/fxceless/victory-fork)** is the
  fork this one starts from: the descriptor fingerprint, the first room, the
  submit thread and the copy queue are its work, carried in the history.
- **[Belobug/VIctory-Belobug-AMD](https://github.com/Belobug/VIctory-Belobug-AMD)**
  and **[uiliannn/VItoriaBr](https://github.com/uiliannn/VItoriaBr)** were read
  for their measurements, not copied from.
- **[ubiquitarianism/leonida](https://github.com/ubiquitarianism/leonida)**
  documented the console's texture layouts and its `layout 0x101` reading.
- **[xenia-canary](https://github.com/xenia-canary/xenia-canary)** is where the
  submission pipeline's shape came from: a frame waits for the frame three
  before it, so the wait is empty.
- **[AnyPS5](https://github.com/boykopovar/AnyPS5)** shows the exact half-wave
  exchange this fork approximates.
- The Xbox Series names, `d3d12_xs`, `xgameruntime` and the RAGE formats were
  read out of the title's own imports and data.