pub fn firmware() -> &'static [u8] {
    #[cfg(not(feature = "fix_fw"))]
    let fw = include_bytes!("../firmware/43439A0.bin");
    #[cfg(feature = "fix_fw")]
    let fw = unsafe { core::slice::from_raw_parts(0x10100000 as *const u8, 224190) };
    fw
}

pub fn clm() -> &'static [u8] {
    #[cfg(not(feature = "fix_fw"))]
    let clm = include_bytes!("../firmware/43439A0_clm.bin");
    #[cfg(feature = "fix_fw")]
    let clm = unsafe { core::slice::from_raw_parts(0x10140000 as *const u8, 4752) };
    clm
}
