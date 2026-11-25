//! The guest's clock: what `QueryPerformanceCounter` answers to the title.
//!
//! A console title measures its own frame time with the counter, and its
//! physics integrates over that interval. On the console the interval is the
//! frame's (33 ms at 30 fps); hosted, the title is handed the real interval,
//! which on a translated run is several times longer. Gravity then acts for
//! that whole interval while buoyancy answers once a tick: a boat sinks under
//! a static water surface and the player drowns before the hull comes back up.
//!
//! With `VICTORY_GUEST_CLOCK_MS=<ms>` (33 is the console's frame) time is
//! advanced by the frames run, not by the wall clock: each present adds the
//! step, and inside a frame the counter creeps towards the next step but no
//! further than the real time since that present. The title therefore sees a
//! frame time of at most one step, and its physics runs at the console's rate
//! while the run goes as fast as the host can draw it. The game runs in slow
//! motion — that is the point: its time, not its frame rate, is what the
//! physics and every scenario timer are written against.
//!
//! Off (0, the default): the counter is the host's, as before.

use std::sync::atomic::{AtomicI64, AtomicU64, Ordering};

/// Virtual ticks at the last frame boundary.
static VIRTUAL: AtomicI64 = AtomicI64::new(0);
/// The real counter reading at the last frame boundary.
static AT_PRESENT: AtomicI64 = AtomicI64::new(0);
/// The frame's length in ticks, virtual. 0: the clock is off.
static STEP_TICKS: AtomicU64 = AtomicU64::new(0);

/// The host's counter, as it is without us in the way.
pub fn host_counter() -> i64 {
    let mut count = 0i64;
    // SAFETY: a plain out-parameter of our own.
    unsafe { windows_sys::Win32::System::Performance::QueryPerformanceCounter(&mut count) };
    count
}

/// The host's counter frequency in ticks a second.
pub fn host_frequency() -> i64 {
    let mut frequency = 0i64;
    // SAFETY: a plain out-parameter of our own.
    unsafe {
        windows_sys::Win32::System::Performance::QueryPerformanceFrequency(&mut frequency)
    };
    frequency.max(1)
}

/// The step in ticks: 0 when the clock is off.
fn step() -> u64 {
    static ONCE: std::sync::OnceLock<u64> = std::sync::OnceLock::new();
    *ONCE.get_or_init(|| {
        let ms = std::env::var("VICTORY_GUEST_CLOCK_MS")
            .ok()
            .and_then(|v| v.trim().parse::<u64>().ok())
            .unwrap_or(0);
        if ms == 0 {
            tracing::info!("the guest clock is off: the title reads the host's counter");
            return 0;
        }
        let ticks = (host_frequency() as u64).saturating_mul(ms) / 1000;
        STEP_TICKS.store(ticks, Ordering::Relaxed);
        let start = host_counter();
        AT_PRESENT.store(start, Ordering::Relaxed);
        tracing::info!(
            ms,
            ticks,
            "the guest clock advances by the frames run: the title sees one frame a step, its physics at the console's rate"
        );
        ticks
    })
}

/// Whether the guest clock is running (`VICTORY_GUEST_CLOCK_MS` above 0).
pub fn on() -> bool {
    step() > 0
}

/// A frame was presented: the guest's clock reaches the next step.
pub fn presented() {
    if !on() {
        return;
    }
    AT_PRESENT.store(host_counter(), Ordering::Relaxed);
    VIRTUAL.fetch_add(step() as i64, Ordering::Relaxed);
}

/// What the title reads as its counter: the steps of the frames run, plus
/// what of the frame in progress the real clock has gone through, so that a
/// title busy-waiting inside a frame still sees time pass.
pub fn counter() -> i64 {
    if !on() {
        return host_counter();
    }
    let stepped = VIRTUAL.load(Ordering::Relaxed);
    let since = host_counter() - AT_PRESENT.load(Ordering::Relaxed);
    let creep = since.clamp(0, step() as i64);
    stepped + creep
}

/// `QueryPerformanceCounter`, as the title calls it.
///
/// # Safety
/// `count` is the title's out-parameter, a writable `LARGE_INTEGER`.
pub unsafe extern "system" fn query_performance_counter(count: *mut i64) -> i32 {
    if count.is_null() {
        return 0;
    }
    // SAFETY: the caller's writable out-parameter.
    unsafe { *count = counter() };
    1
}

/// `QueryPerformanceFrequency`, as the title calls it.
///
/// # Safety
/// `frequency` is the title's out-parameter, a writable `LARGE_INTEGER`.
pub unsafe extern "system" fn query_performance_frequency(frequency: *mut i64) -> i32 {
    if frequency.is_null() {
        return 0;
    }
    // SAFETY: the caller's writable out-parameter.
    unsafe { *frequency = host_frequency() };
    1
}