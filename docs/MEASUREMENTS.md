# Measurements

Every number here was read from the build's own frame line or from a sampled
profile of it, on the machine below. Nothing is estimated.

- Windows 10 22H2 x64, 32 GB of RAM
- NVIDIA GeForce RTX 4070 SUPER (Ada), shader model 6.6 through the Agility SDK
  redist beside the executable
- GTA VI (Xbox Series X|S) 1.0.2.1, Series X profile, 1920x1080 window

The frame line prints, every five seconds:

```
fps, ms/frame (preparing: copies ms/MB, textures ms/MB; GPU; copying back ms/MB),
draws/frame, dropped without a host pipeline, indirect mesh dispatches,
heap buffers passes, texture slots, textures picked by index: uploaded/waiting/
passed over unmapped, depth twins
```

## Where the frame goes

The same scene (the beach, about 1 400–1 700 draws) at three points in this
fork's history. The numbers are milliseconds a frame.

| Component | parent fork | + submit thread, copy queue, deeper queue | + 1080p window, plain writes, narrower flush |
|---|---|---|---|
| preparing | 108 | 58 | 40–82 |
| ├ copies | 37 | 24–35 | 12–35 |
| └ textures | 9 | 6–25 | 5–9 |
| on the GPU | 567 | 78–89 | 50–89 |
| copying back | 430 | 26–33 | 5–30 (4.9 ms on the copy queue) |
| **frame** | **1 200** | **142–183** | **120–183** |
| **fps** | **1** | **5.5–7** | **5–8** |

The heaviest scenes (a cutscene, a driving shot) still reach 3–4 fps and
200–600 ms: the GPU time there is the title's own work, and it is 3 to 5 times
what the same scene costs the console, not the order of magnitude the frame
above shows.

## What each patch moved

Read off the frame line, one variable at a time.

| Patch | Before | After |
|---|---|---|
| 16-bit targets as `B8G8R8A8` instead of `R8G8B8A8` | clipped red 34.4 %, blue 14.2 % | 2.7 % on all three, measured on a capture |
| `VICTORY_WINDOW_SIZE=1920x1080` | GPU 110–120 ms | 50–90 ms |
| `VICTORY_MEMCPY_WRITES` | 49 ms a frame of kernel time writing our own memory, 44 MB/s | one `rep movs` per range |
| flush only the copies a write can have made stale | 29 678 refreshes, 22 274 MB of lengths asked a frame | 18 511 |
| `VICTORY_LATE_WAIT_LISTS=1024` | 111 ms a frame in `WaitForSingleObject`, `on cpu` 3 %; 3–6 fps | 5.5–7 fps, 142–183 ms |
| `VICTORY_ASYNC_SUBMIT=1` + `VICTORY_QUEUE2=1` | copy-back 430 ms | 58 ms, then 4.9 ms on its own queue |
| `VICTORY_WAVE_FORCE32=1` | 7 compute pipelines refused (`E_INVALIDARG`), water flickering | 1 refused, water steady |
| `VICTORY_CARD_BUFFERS_MIN_MB=1`, `..._MB=1024` | 420–450 ms (measured by ztrx on an RTX 4070) | 240–350 ms |

## The profile

`VICTORY_PROFILE=1` writes a sampled profile of the submitting, recording and
worker threads. The run that showed the pipeline's shape:

```
submit        191.5 ms a frame, on cpu 42 %, waiting 55 %   ntdll!ZwWaitForSingleObject 54 %
record x5     154.9 ms a frame, on cpu  5 %, waiting 94 %   ntdll!ZwWaitForSingleObject 90 %
pipelines x8  191.9 ms a frame, on cpu  0.3 %, waiting 99.7 %
```

`queue::release_held_now` was 137.8 ms of that, of which 111 ms was
`WaitForSingleObjectEx` at `on cpu 3 %`: the submitting thread stood still
while the card drew. xenia's shape — a frame waits for the frame three before
it — is what this fork's deeper queue approximates.

Also from that profile: `NtWriteVirtualMemory` 49.5 ms a frame at `on cpu 98 %`,
in `write_mapped`, writing into memory this process owns.

## What does not run

- **Ray-traced light.** The title's indirect diffuse light comes from probes it
  fills by tracing rays, and the tracing does not run. Its resolve program
  leaves next to nothing, so whatever the sun does not reach directly is black.
  This is why interiors, shaded walls and the shaded side of a face are dark.
- **The mesh pipeline** `mesh f5e50eb2108c425b` with `pixel ce3f0432a08f1d35`
  is refused: its mesh outputs exceed the 32 KiB a mesh program may write, the
  pixel program is linked against the full set of signatures, and D3D12 refuses
  the pair.
- **Physics at the host's frame time.** The title measures its frame with
  `QueryPerformanceCounter` and integrates over it, so at 6 fps gravity acts
  for 160 ms a tick while buoyancy answers once: boats sink under a static
  water surface and the player drowns. Advancing the title's clock by frames
  instead (`VICTORY_GUEST_CLOCK_MS`) was built and **crashed the title**
  (`0xc0000005`, write to 0) — it reads that counter consistently with clocks
  this layer does not own. It is off by default.