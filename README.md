# gvas2json

Convert between GVAS and JSON, TOML, YAML.

GVAS is the file format used by many Unreal Engine 4 and Unreal Engine 5 games.

This crate works with games supported by the
[gvas](https://github.com/localcc/gvas/) crate.

## Install

1. [Install Rust](https://rustup.rs/).
2. `cargo install gvas2json`.

## Use

This crate includes six command-line utilities:

- `gvas2json`
- `gvas2toml`
- `gvas2yaml`
- `json2gvas`
- `toml2gvas`
- `yaml2gvas`

Each utility takes up to two files as arguments:

### Read from an input file, write to an output file

```sh
gvas2json slot1.sav -o gvas.json
gvas2toml slot1.sav -o gvas.toml
gvas2yaml slot1.sav -o gvas.yaml
json2gvas gvas.json -o out.sav
toml2gvas gvas.toml -o out.sav
yaml2gvas gvas.yaml -o out.sav
```

### Read from an input file, write to stdout

```sh
gvas2json slot1.sav > gvas.json
gvas2toml slot1.sav > gvas.toml
gvas2yaml slot1.sav > gvas.yaml
```

### Read from stdin, write to an output file

```sh
json2gvas -o out.sav < gvas.json
toml2gvas -o out.sav < gvas.toml
yaml2gvas -o out.sav < gvas.yaml
```

### Read from stdin, write to stdout

```sh
gvas2json < slot1.sav > gvas.json
gvas2toml < slot1.sav > gvas.toml
gvas2yaml < slot1.sav > gvas.yaml
json2gvas < gvas.json > out.sav
toml2gvas < gvas.toml > out.sav
yaml2gvas < gvas.yaml > out.sav
```

### Get help

```sh
gvas2json --help
gvas2toml --help
gvas2yaml --help
json2gvas --help
toml2gvas --help
yaml2gvas --help
```

## Development

### Build with locally-modified gvas library

```sh
cargo add --path ../gvas
```
