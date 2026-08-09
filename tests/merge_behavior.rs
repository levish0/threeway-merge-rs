use threeway_merge::{MergeFavor, MergeOptions, MergeStyle, merge_strings};

#[test]
fn resolves_obvious_clean_merges() {
    let options = MergeOptions::default();

    let identical = merge_strings("base", "same", "same", &options).unwrap();
    assert_eq!(identical.content, "same");
    assert!(identical.is_clean_merge());

    let ours_unchanged = merge_strings("base", "base", "theirs", &options).unwrap();
    assert_eq!(ours_unchanged.content, "theirs");

    let theirs_unchanged = merge_strings("base", "ours", "base", &options).unwrap();
    assert_eq!(theirs_unchanged.content, "ours");
}

#[test]
fn supports_clean_merge_with_empty_native_output() {
    let options = MergeOptions {
        favor: Some(MergeFavor::Ours),
        ..MergeOptions::default()
    };
    let result = merge_strings("base\n", "", "theirs\n", &options).unwrap();

    assert!(result.is_clean_merge());
    assert!(result.content.is_empty());
}

#[test]
fn default_level_matches_git_for_punctuation_only_gaps() {
    let base = " \n \n}\n";
    let ours = "\n \n!\n{\n!\n{\n \n";
    let theirs = "//\na\na\n}\n \n!\n{\n!\n//\n";

    let result = merge_strings(base, ours, theirs, &MergeOptions::default()).unwrap();

    assert_eq!(result.conflicts, 1);
    assert_eq!(result.content.matches("<<<<<<<").count(), 1);
}

#[test]
fn diff3_output_contains_labels_and_base() {
    let options = MergeOptions {
        style: MergeStyle::Diff3,
        base_label: Some("base".to_string()),
        ours_label: Some("ours".to_string()),
        theirs_label: Some("theirs".to_string()),
        ..MergeOptions::default()
    };

    let result = merge_strings("base\n", "ours\n", "theirs\n", &options).unwrap();

    assert!(result.has_conflicts());
    assert!(result.content.contains("<<<<<<< ours\n"));
    assert!(result.content.contains("||||||| base\nbase\n"));
    assert!(result.content.contains(">>>>>>> theirs\n"));
}
