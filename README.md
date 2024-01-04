# ImgCatr 
[![Licence](https://img.shields.io/badge/license-MIT-blue.svg?style=flat)](LICENSE) [![Crates.io version](https://img.shields.io/crates/v/imgcatr)](https://crates.io/crates/imgcatr)

A rust-based command-line command `imgcatr` to display images.

<p align="center">
  <img src="https://github.com/SilinMeng0510/imgcatr/blob/main/assets/sample.png" alt="running `imgcatr cat.png`" width=506 height=620>
</p>


# Install
### Crate.io
__(Recommended)__ Install from __Crate.io__, it's required to have [Cargo](https://www.rust-lang.org/tools/install) on your computer.
```sh
cargo install imgcatr
```
### On MacOS
1. Open Terminal
2. Use curl to Download the Release
```sh
curl -L https://github.com/SilinMeng0510/imgcatr/releases/download/v0.1.4/imgcatr-x86_64-apple-darwin.tar.gz -o imgcatr-x86_64-apple-darwin.tar.gz
```
3. Extract the Tarball
```sh
tar -xzf imgcatr-x86_64-apple-darwin.tar.gz
```
4. Move the Binary to a bin Directory
```sh
mv imgcatr /usr/local/bin/
```
### On Linux
1. Open Terminal
2. Use curl to Download the Release
```sh
curl -L https://github.com/SilinMeng0510/imgcatr/releases/download/v0.1.4/imgcatr-x86_64-unknown-linux-gnu.tar.gz -o imgcatr-x86_64-unknown-linux-gnu.tar.gz
```
3. Extract the Tarball
```sh
tar -xzf imgcatr-x86_64-unknown-linux-gnu.tar.gz
```
4. Move the Binary to a bin Directory
```sh
sudo mv imgcatr /usr/local/bin/
```
### On Windows
1. Using PowerShell (Windows 10 and later):
2. Use the Invoke-WebRequest cmdlet to download:
```sh
Invoke-WebRequest -Uri https://github.com/SilinMeng0510/imgcatr/releases/download/v0.1.4/imgcatr-x86_64-pc-windows-msvc.zip -OutFile imgcatr-windows.zip
```
3. Extract zip file
```sh
Expand-Archive -LiteralPath imgcatr-windows.zip -DestinationPath imgcatr-windows
```
__(Optional)__ Create a directory (if you don't already have one):
```sh
New-Item -ItemType Directory -Force -Path "C:\Tools"
```
Move `imgcatr.exe`:
```sh
Move-Item imgcatr-windows\imgcatr.exe C:\Tools\
```
 

# Usage
### Overview
After installing the command on your computer's bin directory, you can directly call `imgcatr` on CLI with the following instructions to display your images.
```
Usage: imgcatr [OPTIONS] <IMAGE>

Arguments:
  <IMAGE>  Image file to display

Options:
  -s, --size <NxM>   Image size to display [default: 138x22]
  -f, --force        Don't preserve the image's aspect ratio
  -a, --ansi <ANSI>  Force output ANSI escape [possible values: truecolor, simple-black, simple-white, ascii]
  -h, --help         Print help
  -V, --version      Print version
```
There are some features that you can customize how you would like to display the image. You can follow my documentation below to get more information on this command.

### Display Format
With the preparation of the image, you can specify the output format with `-a` or `-ansi`, where Imgcatr has 4 available output formats: `truecolor`, `simple-black`, `simple-white`, `ascii`, and `no_ansi` (For Windows OS).

<p align="center">
  <img src="https://github.com/SilinMeng0510/imgcatr/blob/main/assets/outputs.png" alt="running `imgcatr cat.png`" width=1000 height=700>
</p>

### Customized Size
The default size of the image display is set to the size of the __CLI__. This means that the height of your image will not go beyond the height of your terminal.
However, you can specify the image size with `-s` or `-size` followed by input in `<NxM>` format. An example is provided below, where the image size is set to 100x100.
```sh
imgcatr cat.png --size 100x100
```

### Ratio Preserve
The image ratio is preserved as default. And option `-f` or `-force` can force the program to not preserve the image ratio.
<p align="center">
  <img src="https://github.com/SilinMeng0510/imgcatr/blob/main/assets/preserve-display.png" alt="running `imgcatr cat.png`" width=1000 height=300>
</p>

# Additional Comments
### Special Thanks
This work is derived from an open-sourced project named [`termimage`](https://github.com/nabijaczleweli/termimage). Here, I'm offering my special thanks to the team.

### Comparison
The previous work `termimage` uses an older version of the clap-v2 crate(library). `imgcatr` replaces clap-v2 with the latest implementation of clap-v4, which has better performance overall.
In addition, `imgcatr` offers a new feature in which users can display the ASCII format of images. 

### The Future
There will be potential improvement in future work with faster API and more features available.











