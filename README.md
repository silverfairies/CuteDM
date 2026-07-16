# CuteDM

Innitialy planned as a display manager, this is a TUI application for executing trees of shell scripts. CuteDM is a personal project, so it won't be developed much after achieving what I need to use my computer easier. On the other hand, if there will be any people, that show interest in the project, I may continue working on it.

IMPORTANT: CuteDM is expected to work only on Linux, at least for now. It is not tested on any other *nix systems, but may work and any test information is welcome. I will never try to port it for Windows, althrough it may become compatible "accidentaly", and contributions are welcome.

## Installation

### Binaries
Binaries are available only for Linux (tested under Void Linux) under Releases after the first release. Make sure to have some shell installed.

### From Source
For unsupported platforms or latest features you can compile from source directly. Make sure to have git, cargo and some shell installed.
```sh
git clone https://github.com/silverfairies/CuteDM
cd CuteDM
cargo build --release
```

## Usage

```sh
cutedm /path/to/directory
```
A directory is a tree of executable shell scripts. Each script has the extension .sh and, if it needs further choices, a directory with the same name as the script **without the .sh extension**. Scripts beginning with . are ignored, as well as any files without .sh extension and directories without the coresponding script. If you want to make a sublist without a script you can use ```mkdir list; touch list.sh```. An example of a tree is available in examples.

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

### AI/LLM Usage
No AI/LLM was is being and will ever be used in this project. Any pull requests with major AI/LLM written code will be rejected. Sensible bug reports written/made with the help of an AI/LLM will be taken seriously, but, as this is only a personal project, will not get priority, unless they are actualy important.

## License

[MIT](https://choosealicense.com/licenses/mit/)
