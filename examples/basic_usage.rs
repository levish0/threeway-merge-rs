use std::fs;
use threeway_merge_rs::{DiffAlgorithm, MergeOptions, MergeStyle, merge_strings};

fn main() {
    // Example equivalent to:
    // git merge-file --diff-algorithm histogram --zdiff3 -L "mine" ours.txt -L "original" base.txt -L "theirs" theirs.txt --stdout > result.txt

    let base = fs::read_to_string("examples/base.txt").expect("Failed to read base.txt");
    let ours = fs::read_to_string("examples/ours.txt").expect("Failed to read ours.txt");
    let theirs = fs::read_to_string("examples/theirs.txt").expect("Failed to read theirs.txt");

    let mut options = MergeOptions::default();
    options.algorithm = DiffAlgorithm::Histogram;
    options.style = MergeStyle::ZealousDiff3;
    options.ancestor_label = Some("original".to_string());
    options.ours_label = Some("mine".to_string());
    options.theirs_label = Some("theirs".to_string());

    match merge_strings(&base, &ours, &theirs, &options) {
        Ok(result) => {
            // Write result to file
            fs::write("examples/result.txt", &result.content).expect("Failed to write result.txt");

            println!("Merge result (conflicts: {}):", result.conflicts);
            println!("{}", result.content);

            if result.conflicts > 0 {
                println!(
                    "\n⚠️  Conflicts detected! Check examples/result.txt for manual resolution."
                );
            } else {
                println!("\n✅ Clean merge successful! Result saved to examples/result.txt");
            }
        }
        Err(e) => {
            eprintln!("Merge failed: {}", e);
        }
    }
}
