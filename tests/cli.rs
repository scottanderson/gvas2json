use anyhow::Result;
use assert_cmd::prelude::{CommandCargoExt, OutputAssertExt};
use predicates::prelude::predicate;
use std::{fs, path::PathBuf, process::Command};

#[cfg(windows)]
const ERROR_MISSING_FILE: &str = "Error: The system cannot find the path specified. (os error 3)\n";
#[cfg(not(windows))]
const ERROR_MISSING_FILE: &str = "Error: No such file or directory (os error 2)\n";

fn missing_file(cmd: &str) {
    Command::cargo_bin(cmd)
        .unwrap()
        .arg("test/file/doesnt/exist")
        .assert()
        .failure()
        .stderr(predicate::str::contains(ERROR_MISSING_FILE));
}

#[test]
fn gvas2json_missing_file() {
    missing_file("gvas2json")
}

#[test]
fn gvas2toml_missing_file() {
    missing_file("gvas2toml")
}

#[test]
fn gvas2yaml_missing_file() {
    missing_file("gvas2yaml")
}

#[test]
fn json2gvas_missing_file() {
    missing_file("json2gvas")
}

#[test]
fn toml2gvas_missing_file() {
    missing_file("toml2gvas")
}

#[test]
fn yaml2gvas_missing_file() {
    missing_file("yaml2gvas")
}

fn bin(cmd: &str, case: &str, ext: &str, expect_ext: &str) -> Result<()> {
    Command::cargo_bin(cmd)?
        .arg(format!("resources/test/{case}.{ext}"))
        .assert()
        .success()
        .stdout(fs::read(
            [
                env!("CARGO_MANIFEST_DIR"),
                "resources",
                "test",
                &format!("{case}.{expect_ext}"),
            ]
            .iter()
            .collect::<PathBuf>(),
        )?);
    Ok(())
}

fn gvas2json(case: &str) -> Result<()> {
    bin("gvas2json", case, "sav", "json")
}

fn gvas2toml(case: &str) -> Result<()> {
    bin("gvas2toml", case, "sav", "toml")
}

fn gvas2yaml(case: &str) -> Result<()> {
    bin("gvas2yaml", case, "sav", "yaml")
}

fn json2gvas(case: &str) -> Result<()> {
    bin("json2gvas", case, "json", "sav")
}

fn toml2gvas(case: &str) -> Result<()> {
    bin("toml2gvas", case, "toml", "sav")
}

fn yaml2gvas(case: &str) -> Result<()> {
    bin("yaml2gvas", case, "yaml", "sav")
}

#[test]
fn gvas2json_sample1() {
    gvas2json("sample1").expect("gvas2json(sample1)");
}

#[test]
fn gvas2toml_sample1() {
    gvas2toml("sample1").expect("gvas2toml(sample1)");
}

#[test]
fn gvas2yaml_sample1() {
    gvas2yaml("sample1").expect("gvas2yaml(sample1)");
}

#[test]
fn json2gvas_sample1() {
    json2gvas("sample1").expect("json2gvas(sample1)");
}

#[test]
fn toml2gvas_sample1() {
    toml2gvas("sample1").expect("toml2gvas(sample1)");
}

#[test]
fn yaml2gvas_sample1() {
    yaml2gvas("sample1").expect("yaml2gvas(sample1)");
}

#[test]
fn gvas2json_sample2() {
    gvas2json("sample2").expect("gvas2json(sample2)");
}

#[test]
fn gvas2toml_sample2() {
    gvas2toml("sample2").expect("gvas2toml(sample2)");
}

#[test]
fn gvas2yaml_sample2() {
    gvas2yaml("sample2").expect("gvas2yaml(sample2)");
}

#[test]
fn json2gvas_sample2() {
    json2gvas("sample2").expect("json2gvas(sample2)");
}

#[test]
fn toml2gvas_sample2() {
    toml2gvas("sample2").expect("toml2gvas(sample2)");
}

#[test]
fn yaml2gvas_sample2() {
    yaml2gvas("sample2").expect("yaml2gvas(sample2)");
}
