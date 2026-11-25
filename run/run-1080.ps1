# 1080p window (VICTORY_WINDOW_SIZE patch) + leaders' measured env + profile.
# Built into E:\Build\vf-build so the running game's exe is untouched.
Set-Location 'E:\Build\vf-build\release'

$env:VICTORY_GAME_DIR = 'E:\Build\GTA6.Xbox.v1.0.2.1-CYBERLEEK'
$env:VICTORY_WINDOW_SIZE = '1920x1080'
$env:VICTORY_DISPLAY_MODE = '1920x1080'
$env:VICTORY_MEMORY_MB = '20480'

# pipelining: keep the CPU off the card's neck
$env:VICTORY_LATE_WAIT = '3'
$env:VICTORY_LATE_WAIT_LISTS = '1024'
$env:VICTORY_WIDE_BACK = '1'
$env:VICTORY_ASYNC_SUBMIT = '1'
$env:VICTORY_QUEUE2 = '1'

# big title buffers, ztrx's measured pair on the same card
$env:VICTORY_CARD_BUFFERS_MIN_MB = '1'
$env:VICTORY_CARD_BUFFERS_MB = '1024'
$env:VICTORY_CARD_FETCH_INFLIGHT = '0'

# textures: the plain budget, no catch-up multiplier
$env:VICTORY_TEXTURE_STAGE_MB = '192'
$env:VICTORY_TEXTURE_STAGE_MS = '250'
$env:VICTORY_TEXTURE_CATCHUP = '0'
$env:VICTORY_TEXTURE_NO_WAIT = '1'
$env:VICTORY_TEXTURE_BUDGET_MB = '12000'

$env:VICTORY_PIPELINE_WAIT_MS = '24'
$env:VICTORY_JOIN_COPIES = '1'

# diagnostics: PROFILE does not disable the flights (GPU_TIMING does), and the
# debug layer names the field of the pipeline state that D3D12 refuses.
$env:VICTORY_GUEST_CLOCK_MS = '0'
$env:VICTORY_PROFILE = '1'
$env:VICTORY_WAVE_FORCE32 = '1'
$env:VICTORY_WAVE_WHY = '1'

& '.\victory.exe' run 'E:\Build\GTA6.Xbox.v1.0.2.1-CYBERLEEK\game_scarlett_master.exe' --execute --console series-x