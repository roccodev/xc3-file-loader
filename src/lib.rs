use std::{ffi::CString, sync::OnceLock};

mod arh1;
mod arh2;
mod loader;
mod util;

use anyhow::{bail, Result};
use loader::FileLoader;
use skyline::nn;
use util::{Game, GameConfig};

const GAME_CONFIGS: [(Game, GameConfig); 4] = [
    (
        Game::Xc2,
        GameConfig {
            offset_lookup: 0x006b287c, // Last update: 2.1.0
            top_level_blacklist: &["bf2.arh", "bf2.ard", "stream"],
        },
    ),
    (
        Game::Torna,
        GameConfig {
            offset_lookup: 0x008435c8, // Last update: 1.0.0
            top_level_blacklist: &["ira.arh", "ira.ard", "stream"],
        },
    ),
    (
        Game::Xc3,
        GameConfig {
            offset_lookup: 0x01257798, // Last update: 2.2.0
            top_level_blacklist: &["bf3.ard", "bf3.arh", "movie", "sound"],
        },
    ),
    (
        Game::Xcxde,
        GameConfig {
            offset_lookup: 0x013c9dc0, // Last update: 1.0.1
            top_level_blacklist: &["sts.ard", "sts.arh", "movie", "sound"],
        },
    ),
];

static FILE_LOADER: OnceLock<FileLoader> = OnceLock::new();

#[skyline::main(name = "xcnx_file_loader")]
pub fn main() {
    dbg_println!("[XCNX-Files] Loading...");

    if let Err(e) = run() {
        error_dialog(format!("[xcnx_file_loader] {e}"));
        return;
    }

    dbg_println!("[XCNX-Files] Loaded!");
}

fn run() -> Result<()> {
    let game = Game::detect()?;
    let cfg = GAME_CONFIGS
        .into_iter()
        .find_map(|(g, cfg)| (g == game).then_some(cfg))
        .unwrap();

    unsafe {
        let loader = match FileLoader::import_all(&cfg) {
            Ok(loader) => loader,
            Err(id) => {
                bail!("FS read error: {id}");
            }
        };
        if loader.is_empty() {
            bail!("No files found");
        }
        if FILE_LOADER.set(loader).is_err() {
            panic!("loader already init");
        }
    }

    game.hook(&cfg)?;

    Ok(())
}

fn error_dialog(mut message: String) {
    message.truncate(2047);
    let message = CString::new(message).unwrap();
    let code = (1 << 9) | 168; // 168 = userland error
    unsafe {
        let mut error = nn::err::ApplicationErrorArg::new();
        error.SetApplicationErrorCodeNumber(code);
        error.SetDialogMessage(message.as_ptr() as *const _);
        nn::err::ShowApplicationError(&error as *const _);
    }
}
