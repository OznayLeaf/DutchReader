//! Process-kill test for the atomic writer.
//!
//! The child half of this test rewrites a JSON file in a tight loop. The parent
//! kills it at an arbitrary point, then checks that the file on disk still
//! parses and holds a complete value. This exercises the real failure mode
//! (process death mid-write), not just a hand-made leftover `.tmp` file.

use dutchreader::storage::atomic::{LoadSource, load_json, write_json_atomic};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
struct Counter {
    value: u64,
    padding: String,
}

/// Child half. Does nothing during a normal test run.
#[test]
fn child_writer_loop() {
    let Ok(path) = std::env::var("DUTCHREADER_KILL_WRITER_PATH") else {
        return;
    };
    let path = PathBuf::from(path);
    if let Ok(ready) = std::env::var("DUTCHREADER_KILL_WRITER_READY") {
        let _ = fs::write(ready, b"ready");
    }
    let padding = "x".repeat(16 * 1024);
    for value in 0u64..=u64::MAX {
        write_json_atomic(
            &path,
            &Counter {
                value,
                padding: padding.clone(),
            },
        )
        .expect("child write");
    }
}

fn temp_dir() -> PathBuf {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let n = COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!("dutchreader-kill-{}-{n}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

#[test]
fn killing_the_writer_mid_write_never_corrupts_the_file() {
    let dir = temp_dir();
    let path = dir.join("progress.json");
    let ready = dir.join("ready");
    write_json_atomic(
        &path,
        &Counter {
            value: 0,
            padding: "initial".into(),
        },
    )
    .unwrap();

    for round in 0..3 {
        let _ = fs::remove_file(&ready);
        let mut child = Command::new(std::env::current_exe().unwrap())
            .args(["--exact", "child_writer_loop", "--nocapture"])
            .env("DUTCHREADER_KILL_WRITER_PATH", &path)
            .env("DUTCHREADER_KILL_WRITER_READY", &ready)
            .spawn()
            .expect("spawn child writer");

        // Wait for the child to start writing, then kill it without warning.
        let deadline = Instant::now() + Duration::from_secs(10);
        while !ready.exists() {
            assert!(Instant::now() < deadline, "child writer never started");
            std::thread::sleep(Duration::from_millis(5));
        }
        std::thread::sleep(Duration::from_millis(50 + round * 40));
        child.kill().expect("kill child writer");
        child.wait().expect("wait for child writer");

        let loaded = load_json::<Counter>(&path);
        assert_eq!(
            loaded.source,
            LoadSource::Main,
            "round {round}: main file must still parse after a kill"
        );
        assert!(
            loaded.warning.is_none(),
            "round {round}: unexpected recovery warning: {:?}",
            loaded.warning
        );
        assert!(
            loaded.value.padding == "initial" || loaded.value.padding.len() == 16 * 1024,
            "round {round}: file must contain a complete value"
        );
    }

    let _ = fs::remove_dir_all(&dir);
}
