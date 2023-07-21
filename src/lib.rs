use std::ffi::CStr;

use crate::loader::FileLoader;
use skyline::hook;

static mut FILE_LOADER: Option<FileLoader> = None;

/// Alias for println!, but disabled on release profiles
macro_rules! dbg_println {
    ($($arg:tt)*) => {{
        #[cfg(debug_assertions)]
        println!($($arg)*);
    }};
}

mod hash;
mod loader;

#[cfg(feature = "edit-version")]
mod edit_version {
    use skyline::hooks::InlineCtx;

    static VERSION_TEMPLATE: &[u8] = b"%s %sF\0";

    // Look for usages of nn::oe::GetDisplayVersion
    #[skyline::hook(offset = 0x008411d0, inline)]
    pub unsafe fn edit_version(ctx: &mut InlineCtx) {
        *ctx.registers[1].x.as_mut() = VERSION_TEMPLATE.as_ptr() as u64;
    }
}

// ml::DevFileArchiver::getFileInfo
#[hook(offset = 0x01256be8)]
unsafe fn block_file_load(p1: u64, p2: u32, name: *const u8, p4: u64) -> u32 {
    let file_name = CStr::from_ptr(name as *const _);
    if let Ok(file_name) = file_name.to_str() {
        if FILE_LOADER.as_ref().unwrap().is_blocked(file_name) {
            // By hiding the file from all archives, we make the game look for it in the romfs
            // directories. Priority is given to DLC in descending order, but loading from the base
            // game is also supported, should no DLC romfs have the file.
            dbg_println!("[XC3-Files] Blocking {file_name}");
            return 0;
        }
    }
    call_original!(p1, p2, name, p4)
}

#[skyline::main(name = "xc3_file_loader")]
pub fn main() {
    dbg_println!("[XC3-Files] Loading...");

    unsafe {
        let loader = match FileLoader::import_all() {
            Ok(loader) => loader,
            Err(id) => {
                println!("FS error while reading files: {id}");
                return;
            }
        };
        FILE_LOADER = Some(loader);
    }

    skyline::install_hooks!(block_file_load);
    #[cfg(feature = "edit-version")]
    skyline::install_hook!(edit_version::edit_version);

    dbg_println!("[XC3-Files] Loaded!");
}
