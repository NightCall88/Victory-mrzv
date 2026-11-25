# 10. Grafting Belobug's hoisted clamps

Date:        2026-11-25
Goal:        GPU time per draw. Belobug measured 225 → 80 ms of GPU time a frame
             on a scene of 9 000 indirect draws after hoisting the clamps of a
             run of them into one list.
Changed:     Nothing landed. `git format-patch` from a shallow clone into
             `E:/Build/hoist`, then `git apply --3way` of `ccab973`, `918ab03`
             and `76387c2`.
Measured:    Conflicts in five files: `draw.rs`, `host.rs`, `indirect.rs`,
             `list.rs`, `shader_cache.rs`. Reverted; the tree went back to
             `be376ab`.
Verdict:     FAILURE
Why:         that fork sits on upstream `main`, this one on the fxceless line
             plus `f85101b`, so the same functions differ enough that a
             three-way merge cannot place the hunks. It has to be re-written
             against this tree, not applied to it.
