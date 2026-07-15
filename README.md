# CuteDM

Innitialy planned as a display manager, this is a TUI application for executing trees of shell scripts.

## Installation

### Binaries
Binaries are available only on Linux (tested under Void Linux) under Releases after the first release. Make sure to have some shell installed.

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
A directory is a tree of shell scripts. Each script has the extension .sh and, if it needs further choices, a directory with the same name as the script **without the .sh extension**. Scripts beginning with . are ignored. If you want to make a sublist without a script you can use ```mkdir list; touch list.sh```. An example of a tree is available in examples.

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

## License

[MIT](https://choosealicense.com/licenses/mit/)
