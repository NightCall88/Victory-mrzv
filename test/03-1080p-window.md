# 11. 1920x1080: four times fewer pixels for the same geometry

Date:        2026-11-25
Goal:        the GPU time of a world frame.
Changed:     `crates/victory-hle/src/modules/lifecycle.rs`: `VICTORY_WINDOW_SIZE`
             shrinks the window before it is made.
Measured:    GPU time on a beach scene 110–120 ms → 50–90 ms. The frame line's
             `on the GPU` column, same scene, same draws.
Verdict:     SUCCESS
Why:         a console title sizes its window to its render target, 3840x2160,
             and the swap chain and the title's own targets follow it. Reporting
             a smaller display mode (`VICTORY_DISPLAY_MODE`) changes only what
             the title is told the monitor is; the window, and with it the
             targets, stayed 4K.
