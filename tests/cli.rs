use anyhow::Result;
use assert_cmd::prelude::{CommandCargoExt, OutputAssertExt};
use predicates::prelude::predicate;
use predicates::prelude::predicate::str::contains;
use std::{fs, path::PathBuf, process::Command};

#[test]
fn gvas2json_missing_file() {
    Command::cargo_bin("gvas2json")
        .unwrap()
        .arg("test/file/doesnt/exist")
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "Error: No such file or directory (os error 2)",
        ));
}

#[test]
fn json2gvas_missing_file() {
    Command::cargo_bin("json2gvas")
        .unwrap()
        .arg("test/file/doesnt/exist")
        .assert()
        .failure()
        .stderr(contains("Error: No such file or directory (os error 2)"));
}

fn gvas2json(case: &str) -> Result<()> {
    Command::cargo_bin("gvas2json")?
        .arg(&format!("resources/test/{case}.sav"))
        .assert()
        .success()
        .stdout(fs::read_to_string(
            [
                env!("CARGO_MANIFEST_DIR"),
                "resources",
                "test",
                &format!("{case}.json"),
            ]
            .iter()
            .collect::<PathBuf>(),
        )?);
    Ok(())
}

fn json2gvas(case: &str) -> Result<()> {
    Command::cargo_bin("json2gvas")?
        .arg(&format!("resources/test/{case}.json"))
        .assert()
        .success()
        .stdout(fs::read(
            [
                env!("CARGO_MANIFEST_DIR"),
                "resources",
                "test",
                &format!("{case}.sav"),
            ]
            .iter()
            .collect::<PathBuf>(),
        )?);
    Ok(())
}

#[test]
fn sample1() {
    gvas2json("sample1").expect("gvas2json(sample1)");
    json2gvas("sample1").expect("json2gvas(sample1)");
}
