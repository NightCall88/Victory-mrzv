# 6. A deeper queue before the flights are settled

Date:        2026-11-25
Goal:        the submitting thread standing still. The profile had
             `queue::release_held_now` at 137.8 ms a frame, 111 ms of it
             `WaitForSingleObjectEx` at `on cpu 3 %`: the CPU and the card ran in
             series.
Changed:     `VICTORY_LATE_WAIT_LISTS=1024`, from 256. No code.
Measured:    3–6 fps / 200–600 ms → 5.5–7.0 fps / 142–183 ms, no exceptions, on
             the same beach scene.
Verdict:     SUCCESS
Why:         xenia's shape: no thread waits for the card to finish the frame it
             is in. More host lists may be in the air, so the wait for the oldest
             flight is already met when the next frame opens.
