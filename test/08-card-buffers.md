# 8. The bigger card-buffer budget

Date:        2026-11-25
Goal:        copies of the card's own memory, measured at 4 994 MB a frame across
             all ranges, one 182 MB buffer asked for 16 times a frame.
Changed:     `VICTORY_CARD_BUFFERS_MIN_MB=1`, `VICTORY_CARD_BUFFERS_MB=1024`
             (the pair ztrx measured on an RTX 4070).
Measured:    Copies 40.6 ms / 158 MB → 1.1 ms / 7.7 MB with the buffers out of
             the card entirely; the measured pair sits between that and the
             default and brought 420–450 ms → 240–350 ms in ztrx's own run.
Verdict:     SUCCESS
Why:         a title buffer of 16 MiB or more gets a copy in the card's memory
             and each submission sends it whole again; the ring positions inside
             it change every frame, so almost all of that traffic was
             unchanged bytes.
