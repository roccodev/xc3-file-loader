use std::{ffi::CString, sync::OnceLock};

mod arh1;
mod arh2;
mod loader;
mod util;

use anyhow::{bail, Result};
use loader::FileLoader;
use skyline::nn;
use util::Game;

static FILE_LOADER: OnceLock<FileLoader> = OnceLock::new();

#[skyline::main(name = "xcnx_file_loader")]
pub fn main() {
    println!("[XCNX-Files] Loading...");

    if let Err(e) = run() {
        error_dialog(format!("[xcnx_file_loader] {e}"));
        return;
    }

    println!("[XCNX-Files] Loaded!");
}

fn run() -> Result<()> {
    let game = Game::detect()?;

    unsafe {
        let loader = match FileLoader::import_all() {
            Ok(loader) => loader,
            Err(id) => {
                bail!("FS read error: {id}");
            }
        };
        if loader.is_empty() {
            dbg_println!("No files found");
            return Ok(());
        }
        if let Err(_) = FILE_LOADER.set(loader) {
            panic!("loader already init");
        }
    }

    game.hook()?;

    Ok(())
}

fn error_dialog(mut message: String) {
    message.truncate(2047);
    let message = CString::new(message).unwrap();
    let code = (1 << 9) | 168; // 168 = userland crash
    unsafe {
        let mut error = nn::err::ApplicationErrorArg::new();
        error.SetApplicationErrorCodeNumber(code);
        error.SetDialogMessage(message.as_ptr() as *const _);
        nn::err::ShowApplicationError(&error as *const _);
    }
}
