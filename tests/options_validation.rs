use threeway_merge::{MergeError, MergeOptions, merge_strings};

#[test]
fn rejects_labels_with_nul_byte() {
    let mut options = MergeOptions::default();
    options.base_label = Some("ba\0se".to_string());

    let err = merge_strings("base", "ours", "theirs", &options).unwrap_err();
    assert!(matches!(err, MergeError::InvalidInput(_)));
}

#[test]
fn rejects_marker_size_over_c_int_max() {
    let mut options = MergeOptions::default();
    options.marker_size = (i32::MAX as usize) + 1;

    let err = merge_strings("base", "ours", "theirs", &options).unwrap_err();
    assert!(matches!(err, MergeError::InvalidInput(_)));
}
