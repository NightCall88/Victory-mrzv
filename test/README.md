# test — every run, what it proved

One file per attempt, oldest first. Each says what was changed, the numbers it
was measured at, and whether it worked. A failed attempt is kept: it is what
keeps the next one from repeating it.

Format:

```
# <n>. <what the run tried>
Date:        <when, UTC>
Goal:        <the defect or the number it was aimed at>
Changed:     <the file, the switch, the value>
Measured:    <before → after, read from the frame line or a profile>
Verdict:     SUCCESS | PARTIAL | FAILURE
Why:         <the mechanism, in one or two lines>
```

| # | Attempt | Verdict |
|---|---|---|
| 1 | Agility SDK exported from the executable | SUCCESS |
| 2 | 16-bit full-screen targets keep their channel order | SUCCESS |
| 3 | The window the title asks for can be sized | SUCCESS |
| 4 | Write-backs through a plain move | SUCCESS |
| 5 | A copy flushes only what it can have made stale | PARTIAL |
| 6 | A deeper queue before the flights are settled | SUCCESS |
| 7 | A program needing 64-lane waves runs in waves of 32 | SUCCESS |
| 8 | The bigger card-buffer budget (ztrx's pair) | SUCCESS |
| 9 | The title's clock advanced by frames | FAILURE |
| 10 | Grafting Belobug's hoisted clamps | FAILURE |
| 11 | 1920x1080: four times fewer pixels for the same geometry | SUCCESS |
