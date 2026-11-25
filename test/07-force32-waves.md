# 7. A program needing 64-lane waves runs in waves of 32

Date:        2026-11-25
Goal:        seven compute pipelines refused with `E_INVALIDARG`, the
             screen-tile pass among them; the water flickered and the cutscenes
             did not composit.
Changed:     `crates/victory-shader/src/emit/lower.rs`: `fn force32()`,
             `VICTORY_WAVE_FORCE32=1`, and one condition before `if lds_ok`.
             The LDS split and the half-wave exchange already existed.
Measured:    Pipeline refusals 7 → 1. The water is steady in the picture.
             `VICTORY_WAVE_WHY=1` named the reason per program: "LDS in a group
             over 32, an LDS read and write in one lane-branch region,
             v_readlane".
Verdict:     SUCCESS (approximate)
Why:         a host with 32-lane waves refuses `WaveSize(64)` outright, and a
             program with no pipeline draws nothing at all. Splitting the group
             in two waves of 32 with a barrier before every LDS access keeps the
             console's order; the halves exchange through two dwords. The exact
             form — masked shuffles instead of a barrier split — is what AnyPS5
             and Leonida do in SPIR-V, and what the four programs that still do
             not translate would need.
