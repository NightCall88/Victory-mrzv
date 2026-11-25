//! D3D12.X on host D3D12.
//!
//! D3D12.X records commands from code inlined into the title, which writes
//! packets into the command list while its put pointer is below a limit and
//! calls the list's method otherwise. Our lists keep the limits at zero, so
//! every command arrives here as a call, to be run on host D3D12. The
//! title's GPU addresses are CPU addresses (console memory is unified).
//!
//! [`device`] is the device object titles create, [`dxgi`] its adapter side,
//! [`resource`] heaps and resources, [`fence`] fences, [`queue`] command
//! queues, [`list`] command allocators and lists, [`descriptor`]
//! descriptor heaps, [`root`] root signatures, [`query`] query heaps,
//! [`signature`] command signatures, [`pipeline`] pipeline states,
//! [`frame`] frame pacing, [`object`] what all console graphics objects
//! share, [`console`] which console model the title sees. Planned:
//! `present` (swap chain).

#[cfg(all(windows, feature = "diagnostics"))]
mod capture;
pub mod console;
#[cfg(all(windows, feature = "diagnostics"))]
mod copystat;
pub mod descriptor;
pub mod device;
#[cfg(windows)]
mod draw;
pub mod dxgi;
pub mod fence;
pub mod frame;
#[cfg(windows)]
mod host;
#[cfg(windows)]
mod indirect;
#[cfg(windows)]
mod inplace;
pub mod list;
pub mod object;
pub mod pipeline;
#[cfg(windows)]
mod primitive;
#[cfg(all(windows, feature = "diagnostics"))]
mod profile;
pub mod query;
pub mod queue;
#[cfg(windows)]
mod queue2;
pub mod resource;
pub mod root;
#[cfg(windows)]
mod sampler;
#[cfg(windows)]
pub mod shader_cache;
#[cfg(windows)]
mod memory;
#[cfg(windows)]
mod shadow;
#[cfg(windows)]
mod tiles;
pub mod signature;
#[cfg(windows)]
mod texture;
#[cfg(windows)]
mod watch;

#[cfg(windows)]
pub use host::on_failure;
#[cfg(windows)]
pub use inplace::{title_memory_freeing, title_memory_section, title_memory_view};
#[cfg(windows)]
pub use watch::{
    title_memory_reading, title_memory_remapped, title_memory_remapping, title_memory_written,
    writing as kernel_will_write,
};

// The in-box Windows 10 d3d12.dll offers shader model 6.5 only and refuses
// directly-indexed descriptor heaps; the Agility SDK redist beside the
// executable serves 6.6. d3d12.dll looks for these two exported data
// symbols in the host process image: the version (a number) and the path
// to the redist (an ASCII string relative to the executable).
#[repr(transparent)]
pub struct SdkPath(*const std::ffi::c_char);
unsafe impl Sync for SdkPath {}

#[unsafe(no_mangle)]
pub static mut D3D12SDKVersion: u32 = 620;

#[used]
static SDK_PATH_BYTES: [u8; 9] = [46, 92, 68, 51, 68, 49, 50, 92, 0];

#[unsafe(no_mangle)]
pub static D3D12SDKPath: SdkPath = SdkPath(SDK_PATH_BYTES.as_ptr() as *const std::ffi::c_char);
