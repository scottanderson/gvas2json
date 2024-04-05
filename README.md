# gvas2json

Convert between GVAS and JSON or YAML

## Install

1. [Install Rust](https://rustup.rs/).
2. `cargo install gvas2json`.

## Use

`gvas2json`, `gvas2yaml`, `json2gvas`, and `yaml2gvas` take up to two files as arguments:

### Read from an input file, write to an output file

```sh
gvas2json slot1.sav -o gvas.json
gvas2yaml slot1.sav -o gvas.yaml
json2gvas gvas.json -o out.sav
yaml2gvas gvas.yaml -o out.sav
```

### Read from an input file, write to stdout

```sh
gvas2json slot1.sav > gvas.json
gvas2yaml slot1.sav > gvas.json
```

### Read from stdin, write to an output file

```sh
json2gvas -o out.sav < gvas.json
yaml2gvas -o out.sav < gvas.yaml
```

### Read from stdin, write to stdout

```sh
gvas2json < slot1.sav > gvas.json
gvas2yaml < slot1.sav > gvas.yaml
json2gvas < gvas.json > out.sav
yaml2gvas < gvas.yaml > out.sav
```

### Get help

```sh
gvas2json --help
gvas2yaml --help
json2gvas --help
yaml2gvas --help
```

## Development

### Build with locally-modified gvas library

```sh
cargo add --path ../gvas
```
