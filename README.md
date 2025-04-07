# File replacement mod for the Switch Xenoblade games
This mod allows you to load custom files from RomFS instead of the ARD archive.  
This works with any and no DLC packs installed/loaded.

Check the [releases](https://github.com/roccodev/xcnx-file-loader/releases) for the list of supported games.

> **Important**: While this mod does not deal with persistent data, you are still using it at your own risk. I am
not responsible for anything that could happen to your saves, game, console, account, etc. Also note that replacing
files with this mod in XCXDE disables the online features.

## Usage

### Install the mod
1. Download the latest version of the mod from the [Releases](https://github.com/roccodev/xcnx-file-loader/releases/latest) page, make sure
to get the archive for the game you want.
2. Extract the archive to root of your SD card.

### Replacing files

After installing the mod, you can place your custom files in the `/atmosphere/contents/[GAME ID]/romfs/` folder.  
This mod enables loading files that would normally be overwritten by the game's archives.

## Troubleshooting

### Note for macOS users
When performing file operations on files stored on the SD card, macOS may set an attribute bit known as the "archive" bit.
This attribute is not supported by Atmosphere's LayeredFS, so if **your files are not being replaced**, this might be something
worth looking into. To unset the archive bit, you can use Hekate as described in [this guide](https://gbatemp.net/threads/515258/). I recommend changing only the affected directories, as to not interfere with files created by the console.  
Alternatively you can transfer files via FTP using [ftpd](https://github.com/mtheall/ftpd).


### Check that the mod is loaded

If files are not being replaced, check that the mod is being loaded. 

1. Remove all folders from RomFS except for `skyline`.
2. If the mod is loaded, the warning "No files found" should appear when you start the game.

If the warning does not appear, check that you've followed the steps above. 

### Try the test archive

If the mod is being loaded but files still aren't being replaced or the game is crashing,
check whether the files are the problem.

1. Remove all custom files from RomFS.
2. Download the `test-all` archive from the releases tab, and extract it to your SD card.
    * Note: this will install the mod for all games. Feel free to only extract the parts you need.
3. When you start the game, you should notice that some menu fonts are replaced with Comic Sans.

If you see the new fonts, that means your custom files are likely the problem. If the game still crashes
or you don't see the fonts, please open an [issue](https://github.com/roccodev/xcnx-file-loader/issues/new).

## Build instructions
To build the project, install [Rust](https://rustup.rs/) and run
```sh
./build.sh
```
You will also need `npdmtool` in your `PATH`. Alternatively, you can provide the location for the
`npdmtool` executable with the `NPDMTOOL` environment variable.

## License
This mod is distributed under the terms of the [GPLv3](https://www.gnu.org/licenses/gpl-3.0.html). See [COPYING](COPYING) for details.
