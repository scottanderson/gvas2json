# gvas2json

Convert between GVAS and JSON

## Install

1. [Install Rust](https://rustup.rs/).
2. `cargo install gvas2json`.

## Use

`gvas2json` and `json2gvas` take up to two files as arguments:

### Read from an input file, write to an output file

```sh
gvas2json slot1.sav -o gvas.json
json2gvas gvas.json -o out.sav
```

### Read from an input file, write to stdout

```sh
gvas2json slot1.sav > gvas.json
```

### Read from stdin, write to an output file

```sh
json2gvas -o out.sav < gvas.json
```

### Read from stdin, write to stdout

```sh
gvas2json < slot1.sav > gvas.json
json2gvas < gvas.json > out.sav
```

### Get help

```sh
gvas2json --help
json2gvas --help
```
