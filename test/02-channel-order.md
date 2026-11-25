# 2. 16-bit full-screen targets keep their channel order

Date:        2026-11-25
Goal:        the picture read as a red cast with the bright parts burned out.
Changed:     `crates/victory-gpu/src/resource.rs`, `STAND_INS`:
             `[(86,28),(115,28),(67,10)]` → `[(86,87),(115,87),(67,10)]`.
Measured:    On a capture of the overexposed hall, clipped to >= 250:
             before red 34.4 %, green 20.1 %, blue 14.2 % (mean R 220.9, B 166.5);
             after red 2.7 %, green 2.7 %, blue 2.7 %.
Verdict:     SUCCESS
Why:         86 is B5G5R5A1 and 115 is B4G4R4A4: blue-first. Their stand-in was
             28, R8G8B8A8, red-first, so blue landed in red on every such
             target. 87, B8G8R8A8, keeps the order and widens to eight bits.
