## Build
Run the following:
```
git clone https://github.com/sadnessITS/isaac-console.git
cargo build --release
```
You must have it installed:
* [Git](https://git-scm.com/downloads)
* [Cargo](https://www.rust-lang.org/learn/get-started)

## Usage

### Windows
Just run the program and it will automatically switch the console to a different mode. If necessary, you can specify the path to the `option.ini` file with the startup parameter. Example:
```
.\isaac_console.exe C:\option.ini
```
You will then see a message that the Debug Console mode has been successfully changed. Or an **ERROR** if the game is running or the settings file is not found... Example:
```
ENABLED now! Press Enter to close the window...
```

### Other OS
You have to specify the path to the `option.ini` file with the startup parameter anyway. It won't work without **it**.

## License
The program is covered by the GNU Affero General Public License v3.0. You can familiarize yourself with it [here](https://github.com/sadnessITS/isaac-console/blob/master/LICENSE).