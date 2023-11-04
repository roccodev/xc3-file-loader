use skyline::nn::fs;
use std::borrow::Borrow;
use std::ffi::{CStr, CString};
use std::mem::MaybeUninit;

use crate::hash::{crc32_lowercase, PreHashedSet};

/// Top-level paths to skip when matching romfs paths
static TOP_LEVEL_BLACKLIST: [&str; 5] = ["bf3.ard", "bf3.arh", "movie", "sound", "skyline"];

#[derive(Default)]
pub struct FileLoader {
    block_list: PreHashedSet<u32>,
}

struct DirHandle {
    handle: fs::DirectoryHandle,
}

type Result<T> = std::result::Result<T, u32>;

macro_rules! nn_try {
    ($func:expr) => {
        match $func {
            0 => {}
            code => return Err(code),
        }
    };
}

impl FileLoader {
    pub unsafe fn import_all() -> Result<Self> {
        let mut loader = Self::default();
        loader.import_dir("rom:/", 0)?;
        Ok(loader)
    }

    pub fn is_blocked(&self, file_name: &str) -> bool {
        self.block_list.contains(&crc32_lowercase(file_name))
    }

    unsafe fn import_dir(&mut self, path: &str, level: usize) -> Result<()> {
        let handle = DirHandle::new(
            CString::new(if level != 0 {
                &path[..path.len() - 1]
            } else {
                path
            })
            .unwrap(),
        )?;
        let entry_count = handle.get_entry_count()?;
        let mut new_count = 0i64;
        let mut entries: Vec<fs::DirectoryEntry> = Vec::with_capacity(entry_count);
        nn_try!(fs::ReadDirectory(
            &mut new_count,
            entries.as_mut_ptr() as *mut _,
            handle.inner(),
            entry_count as i64,
        ));
        // fs::ReadDirectory returns the number of entries it's written to the buffer, which is
        // always <= the value that was passed as the last parameter.
        entries.set_len(new_count as usize);
        // Close directory early
        drop(handle);

        // IMPORTANT: keep the reference/.iter() here.
        // The DirectoryEntry struct is very heavy (784 bytes), and our environment is really
        // sensitive wrt stack space, it's very easy to overflow.
        for entry in entries.iter() {
            let ty = entry.type_;
            // The DirectoryEntry struct guarantees that the path is null-terminated
            let name = CStr::from_ptr(entry.name.as_ptr() as *const _);
            let name = name.to_string_lossy();

            if level == 0 && TOP_LEVEL_BLACKLIST.contains(&name.borrow()) {
                continue;
            }

            if ty == fs::DirectoryEntryType_DirectoryEntryType_Directory as u8 {
                self.import_dir(&format!("{path}{name}/"), level + 1)?;
            } else {
                self.register_file(&format!("{path}{name}"));
            }
        }
        Ok(())
    }

    fn register_file(&mut self, path: &str) {
        assert!(path.len() >= 4); // rom:/<file name>
        let path = &path[4..];
        let hash = crc32_lowercase(path);
        dbg_println!("[XC3-Files] Registering {path}");
        if !self.block_list.insert(hash) {
            // The game also uses CRC-32 internally to cache resources. It's likely that
            // the game would have also run into issues with this collision.
            dbg_println!("Hash collision for path {path} ({hash:08X})");
        }
    }
}

impl DirHandle {
    unsafe fn new(path: CString) -> Result<Self> {
        let mut handle: fs::DirectoryHandle = MaybeUninit::zeroed().assume_init();
        nn_try!(fs::OpenDirectory(
            &mut handle as *mut _,
            path.as_ptr() as *const _,
            fs::OpenDirectoryMode_OpenDirectoryMode_All as i32
        ));
        Ok(Self { handle })
    }

    fn inner(&self) -> fs::DirectoryHandle {
        self.handle
    }

    fn get_entry_count(&self) -> Result<usize> {
        let mut count = 0i64;
        unsafe {
            nn_try!(fs::GetDirectoryEntryCount(
                &mut count as *mut _,
                self.handle
            ));
        }
        Ok(count as usize)
    }
}

impl Drop for DirHandle {
    fn drop(&mut self) {
        unsafe { fs::CloseDirectory(self.handle) }
    }
}
