# 1. Agility SDK exported from the executable

Date:        2026-11-25
Goal:        the world failed before it drew: this title uses directly-indexed
             descriptor heaps, and Windows 10's in-box d3d12.dll serves shader
             model 6.5, which refuses them.
Changed:     `crates/victory-gpu/src/lib.rs`, `apps/victory/build.rs`.
             Two exported data symbols: `D3D12SDKVersion` (620) and
             `D3D12SDKPath`, a `#[repr(transparent)]` pointer to the ASCII path
             `.\\D3D12\\`. `D3D12Core.dll` beside the executable.
Measured:    Before: no device, `shader_model=0x65`. After: 6.6, heaps build,
             the title reaches the world.
Verdict:     SUCCESS
Why:         d3d12.dll reads both symbols out of the host process image, not
             out of a DLL. A C shim, `.def` and `/EXPORT` alone each failed with
             0xC0000142; the symbol has to be in the executable, and the path
             has to be a pointer, not a buffer.
