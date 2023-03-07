use std::ffi::CStr;

use skyline::nn::oe;
use skyline::{
    hook,
    hooks::{InlineCtx, Region},
};

mod loader;

// ml::DevFileArchiver::getFileInfo
#[hook(offset = 0x01164f58)]
unsafe fn test_get_file_info(p1: u64, p2: u32, name: *const u8, p4: u64) -> u64 {
    let file_name = CStr::from_ptr(name as *const _);
    let st = file_name.to_string_lossy();
    println!("LOADING: {st}");
    if st.contains("menu.bdat") {
        return 0;
    }
    call_original!(p1, p2, name, p4)
}

#[skyline::main(name = "xc3_file_loader")]
pub fn main() {
    println!("[XC3-Files] Loading...");

    skyline::install_hooks!(test_get_file_info);

    println!("[XC3-Files] Loaded!");
}
