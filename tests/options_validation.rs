use threeway_merge::{MergeError, MergeOptions, MergeStyle, merge_strings};

#[test]
fn rejects_labels_with_nul_byte() {
    let options = MergeOptions {
        base_label: Some("ba\0se".to_string()),
        ..MergeOptions::default()
    };

    let err = merge_strings("base", "ours", "theirs", &options).unwrap_err();
    assert!(matches!(err, MergeError::InvalidInput(_)));
}

#[test]
fn rejects_marker_size_over_c_int_max() {
    let options = MergeOptions {
        marker_size: (i32::MAX as usize) + 1,
        ..MergeOptions::default()
    };

    let err = merge_strings("base", "ours", "theirs", &options).unwrap_err();
    assert!(matches!(err, MergeError::InvalidInput(_)));
}

#[test]
fn rejects_marker_size_that_can_overflow_native_output() {
    let options = MergeOptions {
        marker_size: i32::MAX as usize,
        style: MergeStyle::Diff3,
        ..MergeOptions::default()
    };

    let err = merge_strings("base\n", "ours\n", "theirs\n", &options).unwrap_err();
    assert!(matches!(err, MergeError::InvalidInput(_)));
}
