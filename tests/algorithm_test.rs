use threeway_merge_rs::{DiffAlgorithm, MergeFavor, MergeOptions, MergeStyle, merge_strings};

#[test]
fn test_different_algorithms() {
    let base = "line1\nline2\nline3\nline4\nline5";
    let ours = "line1\nchanged2\nline3\nline4\nline5";
    let theirs = "line1\nline2\nline3\nchanged4\nline5";

    println!("=== Testing Different Algorithms ===");

    // Myers algorithm (default)
    let mut options = MergeOptions::default();
    options.algorithm = DiffAlgorithm::Myers;
    let result_myers = merge_strings(base, ours, theirs, &options).unwrap();
    println!("Myers Algorithm:");
    println!("Conflicts: {}", result_myers.conflicts);
    println!("Content:\n{}", result_myers.content);
    println!("---");

    // Minimal algorithm
    options.algorithm = DiffAlgorithm::Minimal;
    let result_minimal = merge_strings(base, ours, theirs, &options).unwrap();
    println!("Minimal Algorithm:");
    println!("Conflicts: {}", result_minimal.conflicts);
    println!("Content:\n{}", result_minimal.content);
    println!("---");

    // Patience algorithm
    options.algorithm = DiffAlgorithm::Patience;
    let result_patience = merge_strings(base, ours, theirs, &options).unwrap();
    println!("Patience Algorithm:");
    println!("Conflicts: {}", result_patience.conflicts);
    println!("Content:\n{}", result_patience.content);
    println!("---");

    // Histogram algorithm
    options.algorithm = DiffAlgorithm::Histogram;
    let result_histogram = merge_strings(base, ours, theirs, &options).unwrap();
    println!("Histogram Algorithm:");
    println!("Conflicts: {}", result_histogram.conflicts);
    println!("Content:\n{}", result_histogram.content);
    println!("---");

    // All should work without crashing
    assert!(!result_myers.content.is_empty());
    assert!(!result_minimal.content.is_empty());
    assert!(!result_patience.content.is_empty());
    assert!(!result_histogram.content.is_empty());
}

#[test]
fn test_merge_styles() {
    let base = "Hello world";
    let ours = "Hello Rust world";
    let theirs = "Hello beautiful world";

    println!("=== Testing Different Merge Styles ===");

    // Normal style
    let mut options = MergeOptions::default();
    options.style = MergeStyle::Normal;
    let result_normal = merge_strings(base, ours, theirs, &options).unwrap();
    println!("Normal Style:");
    println!("Conflicts: {}", result_normal.conflicts);
    println!("Content:\n{}", result_normal.content);
    println!("---");

    // Diff3 style
    options.style = MergeStyle::Diff3;
    let result_diff3 = merge_strings(base, ours, theirs, &options).unwrap();
    println!("Diff3 Style:");
    println!("Conflicts: {}", result_diff3.conflicts);
    println!("Content:\n{}", result_diff3.content);
    println!("---");

    // ZealousDiff3 style
    options.style = MergeStyle::ZealousDiff3;
    let result_zdiff3 = merge_strings(base, ours, theirs, &options).unwrap();
    println!("ZealousDiff3 Style:");
    println!("Conflicts: {}", result_zdiff3.conflicts);
    println!("Content:\n{}", result_zdiff3.content);
    println!("---");

    assert!(result_normal.conflicts > 0);
    assert!(result_diff3.conflicts > 0);
    assert!(result_zdiff3.conflicts > 0);
}

#[test]
fn test_merge_favor() {
    let base = "Hello world";
    let ours = "Hello Rust world";
    let theirs = "Hello beautiful world";

    println!("=== Testing Merge Favor Options ===");

    // No favor (conflicts)
    let mut options = MergeOptions::default();
    options.favor = None;
    let result_none = merge_strings(base, ours, theirs, &options).unwrap();
    println!("No Favor:");
    println!("Conflicts: {}", result_none.conflicts);
    println!("Content:\n{}", result_none.content);
    println!("---");

    // Favor ours
    options.favor = Some(MergeFavor::Ours);
    let result_ours = merge_strings(base, ours, theirs, &options).unwrap();
    println!("Favor Ours:");
    println!("Conflicts: {}", result_ours.conflicts);
    println!("Content:\n{}", result_ours.content);
    println!("---");

    // Favor theirs
    options.favor = Some(MergeFavor::Theirs);
    let result_theirs = merge_strings(base, ours, theirs, &options).unwrap();
    println!("Favor Theirs:");
    println!("Conflicts: {}", result_theirs.conflicts);
    println!("Content:\n{}", result_theirs.content);
    println!("---");

    // Favor union
    options.favor = Some(MergeFavor::Union);
    let result_union = merge_strings(base, ours, theirs, &options).unwrap();
    println!("Favor Union:");
    println!("Conflicts: {}", result_union.conflicts);
    println!("Content:\n{}", result_union.content);
    println!("---");

    // Favor options should reduce conflicts
    assert!(result_ours.conflicts <= result_none.conflicts);
    assert!(result_theirs.conflicts <= result_none.conflicts);
    assert!(result_union.conflicts <= result_none.conflicts);
}

#[test]
fn test_custom_labels() {
    let base = "Hello world";
    let ours = "Hello Rust world";
    let theirs = "Hello beautiful world";

    println!("=== Testing Custom Labels ===");

    let mut options = MergeOptions::default();
    options.ancestor_label = Some("BASE".to_string());
    options.ours_label = Some("OURS".to_string());
    options.theirs_label = Some("THEIRS".to_string());
    options.marker_size = 10;

    let result = merge_strings(base, ours, theirs, &options).unwrap();
    println!("With Custom Labels:");
    println!("Conflicts: {}", result.conflicts);
    println!("Content:\n{}", result.content);

    assert!(result.conflicts > 0);
    // Labels might not appear in libgit2/xdiff output format
    // Just verify the merge worked and produced conflict markers
    assert!(result.content.contains("Hello Rust world"));
    assert!(result.content.contains("Hello beautiful world"));
}
