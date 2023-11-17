# File replacement mod for Xenoblade 3
This mod allows you to load custom files, replacing the game's defaults.  
This works with any and no DLC packs installed/loaded.

> **Important**: While this mod does not deal with persistent data, you are still using it at your own risk. I am
not responsible for anything that could happen to your saves, game, console, account, etc.

## Usage

### Switch
1. Download the latest version of the mod from the [Releases](https://github.com/RoccoDev/xc3-file-loader/releases/latest) page.
2. Extract the archive to root of your SD card.

#### Note for macOS users
When performing file operations on files stored on the SD card, macOS may set an attribute bit known as the "archive" bit.
This attribute is not supported by Atmosphere's LayeredFS, so if **your files are not being replaced**, this might be something
worth looking into. To unset the archive bit, you can use Hekate as described in [this guide](https://gbatemp.net/threads/515258/). I recommend changing only the affected directories, as to not interfere with files created by the console.  
Alternatively you can transfer files via FTP using [ftpd](https://github.com/mtheall/ftpd).

### Ryujinx
1. Download the latest version of the mod from the [Releases](https://github.com/RoccoDev/xc3-file-loader/releases/latest) page.
2. Open Ryujinx, then right-click on the game and select "Open Atmosphere Mods Directory".
3. From the archive, extract the `exefs` and `romfs` directory into the folder you opened.

#### Replacing files

After installing the mod, you can place your custom files in the `/atmosphere/contents/010074f013262000/romfs/` folder.  
This mod enables loading files that would normally be overwritten by the game's archives.

## Build instructions
To build the project, install [Rust](https://rustup.rs/) and run
```sh
./build.sh
```

## License
This mod is distributed under the terms of the [GPLv3](https://www.gnu.org/licenses/gpl-3.0.html). See [COPYING](COPYING) for details.
