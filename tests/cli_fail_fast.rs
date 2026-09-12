//! Fail-fast CLI behavior: unknown bare commands and `wrap log` without a
//! session must error immediately instead of booting a VM.

use std::{
    path::PathBuf,
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

fn fresh_workspace(name: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock")
        .as_nanos();
    let dir = std::env::temp_dir().join(format!("wrap-{name}-{}-{nanos}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("create temporary workspace");
    dir
}

fn wrap() -> Command {
    Command::new(env!("CARGO_BIN_EXE_wrap"))
}

#[test]
fn unknown_bare_command_fails_without_booting() {
    let workspace = fresh_workspace("unknown");
    let output = wrap()
        .current_dir(&workspace)
        .args(["definitely-not-a-wrap-command"])
        .output()
        .expect("run wrap");
    assert!(
        !output.status.success(),
        "unknown command must fail, got {output:?}"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("unknown command"),
        "stderr names the problem: {stderr}"
    );
    assert!(
        stderr.contains("wrap --"),
        "stderr points at the guest escape hatch: {stderr}"
    );
    assert!(
        !stderr.contains("setting up project vm"),
        "must fail before any VM work: {stderr}"
    );
}

#[test]
fn log_without_session_fails_cleanly() {
    let workspace = fresh_workspace("nolog");
    let output = wrap()
        .current_dir(&workspace)
        .args(["log"])
        .output()
        .expect("run wrap");
    assert!(
        !output.status.success(),
        "`wrap log` without a session must fail, got {output:?}"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("no wrap session"),
        "stderr explains there is nothing to read: {stderr}"
    );
}

#[test]
fn log_help_lists_options() {
    let output = wrap().args(["log", "--help"]).output().expect("run wrap");
    assert!(output.status.success(), "help must succeed: {output:?}");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("--tail"), "help shows --tail: {stdout}");
    assert!(stdout.contains("--follow"), "help shows --follow: {stdout}");
}
