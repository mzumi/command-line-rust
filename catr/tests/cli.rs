use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::*;
use rand::{distributions::Alphanumeric, Rng};
use std::fs;

const RPG: &str = "catr";

fn gen_bad_file() -> String {
    loop {
        let filename: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();

        if fs::metadata(&filename).is_err() {
            return filename;
        }
    }
}

#[test]
fn skips_bad_file() -> Result<()> {
    let bad = gen_bad_file();
    let expected = format!("{}: .* [(]os error 2[)]", bad);

    Command::cargo_bin(RPG)?
        .arg(&bad)
        .assert()
        .success()
        .stderr(predicate::str::is_match(expected)?);

    Ok(())
}
