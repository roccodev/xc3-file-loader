use std::{ffi::CString, sync::OnceLock};

use anyhow::{bail, Result};
use skyline::{hooks::Region, libc::c_void};

use crate::{arh1, arh2};

/// Alias for println!, but disabled on release profiles
#[macro_export]
macro_rules! dbg_println {
    ($($arg:tt)*) => {{
        #[cfg(debug_assertions)]
        println!($($arg)*);
    }};
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Game {
    Xc2,
    Torna,
    Xcde,
    Xc3,
    Xcxde,
}

impl Game {
    pub fn detect() -> Result<Game> {
        Ok(match skyline::info::get_program_id() {
            0x0100E95004038000 | 0x0100F3400332C000 => Game::Xc2, // western / asia
            0x0100C9F009F7A000 => Game::Torna,
            0x0100FF500E34A000 => Game::Xcde,
            0x010074F013262000 => Game::Xc3,
            id => bail!("unknown app id {id:016X}"),
        })
    }

    pub fn hook(&self) -> Result<()> {
        match *self {
            g @ Game::Xcxde => arh2::hook(g),
            g => arh1::hook(g),
        }
    }
}

pub fn lookup_symbol(name: &str) -> Option<usize> {
    let mut out = 0usize;
    let name = CString::new(name).unwrap();
    let res = unsafe { skyline::nn::ro::LookupSymbol(&raw mut out, name.as_ptr() as *const _) };
    let text = unsafe { skyline::hooks::getRegionAddress(Region::Text) } as usize;
    if out > text {
        out -= text;
    }
    (res == 0).then_some(out)
}

pub unsafe fn hook_from_text<O>(offset: usize, hook: *const c_void, orig_out: &OnceLock<O>) {
    unsafe {
        let text = skyline::hooks::getRegionAddress(Region::Text) as *const u8;
        let mut orig_addr = std::ptr::null_mut::<c_void>();
        skyline::hooks::A64HookFunction(
            text.add(offset) as *const c_void,
            hook,
            &raw mut orig_addr,
        );
        if let Err(_) = orig_out.set(std::mem::transmute_copy(&orig_addr)) {
            panic!("hook already init");
        }
    }
}
