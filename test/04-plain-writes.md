# 4. Write-backs through a plain move

Date:        2026-11-25
Goal:        `NtWriteVirtualMemory`, 49.5 ms a frame in the sampled profile at
             `on cpu 98 %`, 44 MB/s, writing into memory this process owns.
Changed:     `crates/victory-gpu/src/host.rs`, `write_mapped`:
             `ptr::copy_nonoverlapping` by default, the syscall only on
             `VICTORY_MEMCPY_WRITES=0`.
Measured:    Before: 49.5 ms a frame of kernel time. After: the profile no
             longer shows the call in its top forty.
Verdict:     SUCCESS
Why:         the destination is this process's own memory and the caller opens
             its guard pages first, so a system call cannot beat a `rep movs`.
