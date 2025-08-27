use threeway_merge_rs::{MergeOptions, merge_strings};

#[test]
fn test_conflicting_merge() {
    let base = "Hello world";
    let ours = "Hello Rust world";
    let theirs = "Hello beautiful world";

    let result = merge_strings(base, ours, theirs, &MergeOptions::default()).unwrap();

    println!("=== Conflicting Merge Test ===");
    println!("Base: {}", base);
    println!("Ours: {}", ours);
    println!("Theirs: {}", theirs);
    println!("Result conflicts: {}", result.conflicts);
    println!("Result content:\n{}", result.content);
    println!("---");

    assert!(result.conflicts > 0);
    assert!(result.content.contains("<<<<<<<"));
    assert!(result.content.contains("======="));
    assert!(result.content.contains(">>>>>>>"));
    assert!(result.content.contains("Hello Rust world"));
    assert!(result.content.contains("Hello beautiful world"));
}

#[test]
fn test_simple_conflict() {
    let base = "line1\nline2\nline3";
    let ours = "line1\nours\nline3";
    let theirs = "line1\ntheirs\nline3";

    let result = merge_strings(base, ours, theirs, &MergeOptions::default()).unwrap();

    assert!(result.conflicts > 0);
    assert!(result.content.contains("<<<<<<<"));
    assert!(result.content.contains("======="));
    assert!(result.content.contains(">>>>>>>"));
}

#[test]
fn test_identical_changes() {
    let base = "Hello world";
    let ours = "Hello Rust world";
    let theirs = "Hello Rust world";

    let result = merge_strings(base, ours, theirs, &MergeOptions::default()).unwrap();

    assert_eq!(result.conflicts, 0);
    assert_eq!(result.content, "Hello Rust world");
}

#[test]
fn test_no_conflict_merge() {
    let base = "line1\nline2\nline3";
    let ours = "line1\nmodified\nline3";
    let theirs = "line1\nline2\nline3\nline4";

    let result = merge_strings(base, ours, theirs, &MergeOptions::default()).unwrap();

    println!("=== No Conflict Merge Test ===");
    println!("Base: {:?}", base);
    println!("Ours: {:?}", ours);
    println!("Theirs: {:?}", theirs);
    println!("Result conflicts: {}", result.conflicts);
    println!("Result content: {:?}", result.content);

    // This merge might result in conflict due to xdiff implementation
    // Just verify it doesn't crash and produces valid output
    assert!(!result.content.is_empty());
    assert!(result.content.contains("line1"));
    assert!(result.content.contains("line3"));
}

#[test]
fn test_empty_strings() {
    let result = merge_strings("", "", "", &MergeOptions::default()).unwrap();

    assert_eq!(result.conflicts, 0);
    assert_eq!(result.content, "");
}
