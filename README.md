# File replacement mod for the Switch Xenoblade games
This mod allows you to load custom files from RomFS instead of the ARD archive.  
This works with any and no DLC packs installed/loaded.

Check the [releases](https://github.com/roccodev/xcnx-file-loader/releases) for the list of supported games.

> **Important**: While this mod does not deal with persistent data, you are still using it at your own risk. I am
not responsible for anything that could happen to your saves, game, console, account, etc. Also note that replacing
files with this mod in XCXDE disables the online features.

## Usage

### Switch
1. Download the latest version of the mod from the [Releases](https://github.com/roccodev/xcnx-file-loader/releases/latest) page, make sure
to get the archive for the game you want.
2. Extract the archive to root of your SD card.

#### Note for macOS users
When performing file operations on files stored on the SD card, macOS may set an attribute bit known as the "archive" bit.
This attribute is not supported by Atmosphere's LayeredFS, so if **your files are not being replaced**, this might be something
worth looking into. To unset the archive bit, you can use Hekate as described in [this guide](https://gbatemp.net/threads/515258/). I recommend changing only the affected directories, as to not interfere with files created by the console.  
Alternatively you can transfer files via FTP using [ftpd](https://github.com/mtheall/ftpd).

### Replacing files

After installing the mod, you can place your custom files in the `/atmosphere/contents/[GAME ID]/romfs/` folder.  
This mod enables loading files that would normally be overwritten by the game's archives.

## Build instructions
To build the project, install [Rust](https://rustup.rs/) and run
```sh
./build.sh
```
You will also need `npdmtool` in your `PATH`. Alternatively, you can provide the location for the
`npdmtool` executable with the `NPDMTOOL` environment variable.

## License
This mod is distributed under the terms of the [GPLv3](https://www.gnu.org/licenses/gpl-3.0.html). See [COPYING](COPYING) for details.
