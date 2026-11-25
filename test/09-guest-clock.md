# 9. The title's clock advanced by frames

Date:        2026-11-25
Goal:        boats sinking under a static water surface and the player drowning.
             The title measures its frame with `QueryPerformanceCounter` and
             integrates its physics over that interval: at 6 fps gravity acts
             for 160 ms a tick while buoyancy answers once.
Changed:     `crates/victory-gpu/src/guest_clock.rs` (new),
             `crates/victory-hle/src/lib.rs` (`fn guest_time`, an import hook on
             the counter and its frequency), `Host::present` calls `presented()`.
             `VICTORY_GUEST_CLOCK_MS=33`.
Measured:    With the clock on: `0xc0000005`, write to 0, inside the title's own
             code, inside the world. With `VICTORY_GUEST_CLOCK_MS=0`, the same
             scene and the same switches: no exceptions.
Verdict:     FAILURE
Why:         the title reads that counter consistently with clocks this layer
             does not own — its frame timing, its scenario timers and its
             streaming timeouts agree with each other, and advancing one breaks
             the agreement. The switch is kept, off by default.
