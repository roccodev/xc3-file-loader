use std::{ffi::CStr, sync::OnceLock};

use anyhow::{bail, Result};
use skyline::libc::c_void;

use crate::{
    dbg_println,
    util::{hook_from_text, lookup_symbol, Game},
};

static FILE_INFO_ORIG: OnceLock<
    unsafe extern "C" fn(*const c_void, u32, *const i8, *mut c_void) -> bool,
> = OnceLock::new();

// ml::DevFileArchiver::getFileInfo(this, ml::MEDIA, char const*, ml::DevFileArchiver::FileInfoResult&) const
unsafe extern "C" fn hook_get_file_info(
    this: *const c_void,
    media: u32,
    name: *const i8,
    res: *mut c_void,
) -> bool {
    let file_name = CStr::from_ptr(name as *const _);
    if let Ok(file_name) = file_name.to_str() {
        #[allow(static_mut_refs)]
        if crate::FILE_LOADER.get().unwrap().is_blocked(file_name) {
            // By hiding the file from all archives, we make the game look for it in the romfs
            // directories. Priority is given to DLC in descending order, but loading from the base
            // game is also supported, should no DLC romfs have the file.
            dbg_println!("[XCNX-ARH1-Files] Blocking {file_name}");
            return false;
        }
    }
    unsafe { FILE_INFO_ORIG.get().unwrap()(this, media, name, res) }
}

pub fn hook(game: Game) -> Result<()> {
    let file_info_offset = {
        // For games with symbols, use symbol lookup
        if let Some(from_sym) = lookup_symbol(
            "_ZNK2ml15DevFileArchiver11getFileInfoENS_5MEDIAEPKcRNS0_14FileInfoResultE",
        ) {
            from_sym
        } else {
            match game {
                Game::Xc2 => todo!(),
                Game::Xcde => todo!(),
                Game::Xc3 => 0x01257798,
                _ => bail!("(arh1) no supported method of hooking fileinfo"),
            }
        }
    };
    unsafe {
        hook_from_text(
            file_info_offset,
            hook_get_file_info as *const c_void,
            &FILE_INFO_ORIG,
        );
    }
    Ok(())
}
