use std::{ffi::CStr, sync::OnceLock};

use anyhow::Result;
use skyline::libc::c_void;

use crate::{
    dbg_println, error_dialog,
    util::{hook_from_text, lookup_symbol, Game, GameConfig},
};

static FILE_INFO_ORIG: OnceLock<
    unsafe extern "C" fn(*const c_void, *const i8, *mut c_void) -> bool,
> = OnceLock::new();

unsafe extern "C" fn hook_get_file_info(
    this: *const c_void,
    name: *const i8,
    res: *mut c_void,
) -> bool {
    let file_name = CStr::from_ptr(name);
    if let Ok(file_name) = file_name.to_str() {
        if crate::FILE_LOADER.get().unwrap().is_blocked(file_name) {
            dbg_println!("[XCNX-ARH2-Files] Blocking {file_name}");
            return false;
        }
    }
    unsafe { FILE_INFO_ORIG.get().unwrap()(this, name, res) }
}

unsafe extern "C" fn hook_network_check(_client: u32) -> bool {
    error_dialog(
        "[xcnx_file_loader] Online functionality is disabled, because modded files are loaded."
            .to_string(),
    );
    false
}

pub fn hook(game: Game, config: &GameConfig) -> Result<()> {
    unsafe {
        hook_from_text(
            config.offset_lookup,
            hook_get_file_info as *const c_void,
            Some(&FILE_INFO_ORIG),
        );
        if game == Game::Xcxde {
            if let Some(net) =
                lookup_symbol("_ZN2nn4nifm28IsAnyInternetRequestAcceptedENS0_8ClientIdE")
            {
                hook_from_text::<()>(net, hook_network_check as *const c_void, None);
            }
        }
    }
    Ok(())
}
