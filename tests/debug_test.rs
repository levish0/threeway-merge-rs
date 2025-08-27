use threeway_merge_rs::{merge_strings, MergeOptions};

#[test]
fn debug_no_conflict_merge() {
    let base = "line1\nline2\nline3";
    let ours = "line1\nmodified\nline3";
    let theirs = "line1\nline2\nline3\nline4";

    let result = merge_strings(base, ours, theirs, &MergeOptions::default()).unwrap();
    
    println!("Result content: {:?}", result.content);
    println!("Conflicts: {}", result.conflicts);
    
    // Just check that it doesn't crash for now
    assert!(result.content.contains("line1"));
}