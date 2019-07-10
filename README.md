## Zip lightning

Simple implementation zip bomb algorithm

### Install
```shell
git clone git@github.com:Harzu/zip-lightning.git
cd zip-lightning
make install
```

## Usage
### Global Help
```shell
zip-lightning --help

# Output
Cli for packing zip-bomb

USAGE:
    zip-lightning [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help    Prints this message or the help of the given subcommand(s)
    pack    pack zip bomb
```

### PACK
```shell
zip-lightning pack --help
pack zip bomb

USAGE:
    zip-lightning pack [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -n, --filename <name>    name output file
    -r, --rounds <rounds>    archive rounds
    -s, --size <size>        file size in bytes
```