# 5. A copy flushes only what it can have made stale

Date:        2026-11-25
Goal:        copies costing 24–40 ms and 130–160 MB a frame.
Changed:     `crates/victory-gpu/src/host.rs`: `title_written_at(dst, len)` next
             to `title_written`; a recorded copy flushes only the pending
             refreshes that overlap it.
Measured:    Refreshes a frame 29 678 → 18 511; the lengths asked 22 274 MB → a
             fraction of it. Copies stayed at 24–35 ms, so the rest is elsewhere.
Verdict:     PARTIAL
Why:         every one of a submission's ~20 000 recorded copies used to flush
             every pending refresh, so the same 945 MB was re-checked about a
             dozen times. Deduplication helped, but the remaining refreshes are
             still one per distinct buffer the title touches.
