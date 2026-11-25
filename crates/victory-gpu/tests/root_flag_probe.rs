// Probe: which root-signature shapes the runtime here refuses.
use windows::core::*;
use windows::Win32::Graphics::Direct3D12::*;

fn try_flags(flags: D3D12_ROOT_SIGNATURE_FLAGS) {
    let desc = D3D12_VERSIONED_ROOT_SIGNATURE_DESC {
        Version: D3D_ROOT_SIGNATURE_VERSION_1_1,
        Anonymous: D3D12_VERSIONED_ROOT_SIGNATURE_DESC_0 {
            Desc_1_1: D3D12_ROOT_SIGNATURE_DESC1 {
                NumParameters: 0,
                pParameters: std::ptr::null(),
                NumStaticSamplers: 0,
                pStaticSamplers: std::ptr::null(),
                Flags: flags,
            },
        },
    };
    unsafe {
        let mut blob: Option<ID3DBlob> = None;
        let mut err: Option<ID3DBlob> = None;
        match D3D12SerializeVersionedRootSignature(&desc, &mut blob, Some(&mut err)) {
            Ok(()) => println!("  ok: flags={:#x}", flags.0),
            Err(e) => {
                let text = err.map(|b| {
                    let s = std::slice::from_raw_parts(
                        b.GetBufferPointer().cast::<u8>(),
                        b.GetBufferSize(),
                    );
                    String::from_utf8_lossy(s).into_owned()
                });
                println!("  FAIL flags={:#x}: {e} :: {}", flags.0, text.unwrap_or_default());
            }
        }
    }
}

#[test]
fn probe() {
    let base = D3D12_ROOT_SIGNATURE_FLAG_ALLOW_INPUT_ASSEMBLER_INPUT_LAYOUT;
    for (name, f) in [
        ("base", base),
        ("none", D3D12_ROOT_SIGNATURE_FLAG_NONE),
        ("cbv_direct", D3D12_ROOT_SIGNATURE_FLAG_CBV_SRV_UAV_HEAP_DIRECTLY_INDEXED),
        ("sampler_direct", D3D12_ROOT_SIGNATURE_FLAG_SAMPLER_HEAP_DIRECTLY_INDEXED),
        (
            "all",
            base | D3D12_ROOT_SIGNATURE_FLAG_CBV_SRV_UAV_HEAP_DIRECTLY_INDEXED
                | D3D12_ROOT_SIGNATURE_FLAG_SAMPLER_HEAP_DIRECTLY_INDEXED,
        ),
        (
            "direct_only",
            D3D12_ROOT_SIGNATURE_FLAG_CBV_SRV_UAV_HEAP_DIRECTLY_INDEXED
                | D3D12_ROOT_SIGNATURE_FLAG_SAMPLER_HEAP_DIRECTLY_INDEXED,
        ),
    ] {
        print!("{name}:");
        try_flags(f);
    }
    // And a device, for the tier.
    unsafe {
        let mut device: Option<ID3D12Device> = None;
        let hr = D3D12CreateDevice(None, D3D_FEATURE_LEVEL_12_0, &mut device);
        println!("device: {hr:?}");
        if let Some(d) = device {
            let mut o: D3D12_FEATURE_DATA_D3D12_OPTIONS = Default::default();
            let _ = d.CheckFeatureSupport(
                D3D_FEATURE_D3D12_OPTIONS,
                (&raw mut o).cast(),
                size_of_val(&o) as u32,
            );
            println!("tier: {}", o.ResourceBindingTier.0);
        }
    }
}