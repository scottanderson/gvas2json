use anyhow::Result;
use assert_cmd::prelude::{CommandCargoExt, OutputAssertExt};
use std::{fs, path::PathBuf, process::Command};

const NO_SUCH_FILE_OR_DIRECTORY: &str = "Error: No such file or directory (os error 2)\n";

#[test]
fn gvas2json_missing_file() {
    Command::cargo_bin("gvas2json")
        .unwrap()
        .arg("test/file/doesnt/exist")
        .assert()
        .failure()
        .stderr(NO_SUCH_FILE_OR_DIRECTORY);
}

#[test]
fn json2gvas_missing_file() {
    Command::cargo_bin("json2gvas")
        .unwrap()
        .arg("test/file/doesnt/exist")
        .assert()
        .failure()
        .stderr(NO_SUCH_FILE_OR_DIRECTORY);
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
