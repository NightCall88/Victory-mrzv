// Agility SDK opt-in: export the two data symbols d3d12.dll looks for in the host image.
fn main() {
    println!("cargo:rustc-link-arg=/EXPORT:D3D12SDKVersion,DATA");
    println!("cargo:rustc-link-arg=/EXPORT:D3D12SDKPath,DATA");
    println!("cargo:rerun-if-changed=build.rs");
}
