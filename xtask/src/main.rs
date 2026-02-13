use std::process::{exit, Command};
use std::thread::sleep;
use std::time::Duration;

const CRATES: &[&str] = &["threeway_merge"];

fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.get(1).map(|s| s.as_str()) {
        Some("publish") => publish(false),
        Some("publish-dry") => publish(true),
        _ => {
            eprintln!("Usage: cargo xtask <command>");
            eprintln!();
            eprintln!("Commands:");
            eprintln!("  publish      Publish crates to crates.io");
            eprintln!("  publish-dry  Dry run publish");
            exit(1);
        }
    }
}

fn publish(dry_run: bool) {
    println!("Publishing crates...\n");

    for (i, crate_name) in CRATES.iter().enumerate() {
        println!("Publishing {}...", crate_name);

        let mut cmd = Command::new("cargo");
        cmd.arg("publish").arg("-p").arg(crate_name);

        if dry_run {
            cmd.arg("--dry-run");
        }

        let status = cmd.status().expect("Failed to execute cargo publish");

        if !status.success() {
            eprintln!("Failed to publish {}", crate_name);
            exit(1);
        }

        println!("{} published successfully\n", crate_name);

        if !dry_run && i < CRATES.len() - 1 {
            println!("Waiting 15s for crates.io index sync...");
            sleep(Duration::from_secs(15));
        }
    }

    println!("All crates published!");
}
