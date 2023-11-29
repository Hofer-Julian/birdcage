use std::os::unix::fs as unixfs;
use std::path::PathBuf;
use std::process::Command;

use birdcage::{Birdcage, Exception, Sandbox};
use serde::{Deserialize, Serialize};

use crate::TestSetup;

#[derive(Serialize, Deserialize)]
struct TestData {
    symlink_dir_exec: PathBuf,
}

pub fn setup() -> TestSetup {
    // Create symlinked executable dir.
    let tempdir = tempfile::tempdir().unwrap().into_path();
    let symlink_dir = tempdir.join("bin");
    let symlink_dir_exec = symlink_dir.join("true");
    unixfs::symlink("/usr/bin", &symlink_dir).unwrap();

    let mut sandbox = Birdcage::new();
    sandbox.add_exception(Exception::ExecuteAndRead(symlink_dir_exec.clone())).unwrap();

    // Serialize test data.
    let data = TestData { symlink_dir_exec };
    let data = serde_json::to_string(&data).unwrap();

    TestSetup { sandbox, data }
}

pub fn validate(data: String) {
    // Deserialize test data.
    let data: TestData = serde_json::from_str(&data).unwrap();

    // Ensure symlinked dir's executable works.
    let cmd = Command::new(data.symlink_dir_exec).status().unwrap();
    assert!(cmd.success());

    // Ensure original dir's executable works.
    let cmd = Command::new("/usr/bin/true").status().unwrap();
    assert!(cmd.success());
}