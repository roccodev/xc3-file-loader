use anyhow::Result;
use skyline::libc::c_void;

use crate::util::Game;

unsafe fn hook_hash_lookup(this: *const c_void, name: *const i8, res: *mut c_void) {}

pub fn hook(game: Game) -> Result<()> {
    Ok(())
}
