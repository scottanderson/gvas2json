use anyhow::Result;
use assert_cmd::prelude::{CommandCargoExt, OutputAssertExt};
use predicates::prelude::predicate;
use std::{fs, path::PathBuf, process::Command};

#[cfg(windows)]
const ERROR_MISSING_FILE: &str = "Error: The system cannot find the path specified. (os error 3)\n";
#[cfg(not(windows))]
const ERROR_MISSING_FILE: &str = "Error: No such file or directory (os error 2)\n";

#[test]
fn gvas2json_missing_file() {
    Command::cargo_bin("gvas2json")
        .unwrap()
        .arg("test/file/doesnt/exist")
        .assert()
        .failure()
        .stderr(predicate::str::contains(ERROR_MISSING_FILE));
}

#[test]
fn json2gvas_missing_file() {
    Command::cargo_bin("json2gvas")
        .unwrap()
        .arg("test/file/doesnt/exist")
        .assert()
        .failure()
        .stderr(predicate::str::contains(ERROR_MISSING_FILE));
}

fn gvas2json(case: &str) -> Result<()> {
    Command::cargo_bin("gvas2json")?
        .arg(&format!("resources/test/{case}.sav"))
        .assert()
        .success()
        .stdout(predicate::str::contains(fs::read_to_string(
            [
                env!("CARGO_MANIFEST_DIR"),
                "resources",
                "test",
                &format!("{case}.json"),
            ]
            .iter()
            .collect::<PathBuf>(),
        )?));
    Ok(())
}

fn json2gvas(case: &str) -> Result<()> {
    let ext = "json";
    let expect_ext = "sav";
    Command::cargo_bin("json2gvas")?
        .arg(&format!("resources/test/{case}.{ext}"))
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

#[test]
fn gvas2json_sample1() {
    gvas2json("sample1").expect("gvas2json(sample1)");
}

#[test]
fn json2gvas_sample1() {
    json2gvas("sample1").expect("json2gvas(sample1)");
}
