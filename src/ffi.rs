use std::os::raw::{c_char, c_int, c_long, c_void};

// The C shim intentionally exposes only primitive C types. Keeping xdiff's
// internal structs on the C side avoids duplicating their layout in Rust.
unsafe extern "C" {
    pub fn threeway_merge_run(
        base: *const c_char,
        base_size: c_long,
        ours: *const c_char,
        ours_size: c_long,
        theirs: *const c_char,
        theirs_size: c_long,
        algorithm: c_int,
        marker_size: c_int,
        level: c_int,
        favor: c_int,
        style: c_int,
        base_label: *const c_char,
        ours_label: *const c_char,
        theirs_label: *const c_char,
        result_ptr: *mut *mut c_char,
        result_size: *mut c_long,
    ) -> c_int;

    pub fn threeway_merge_free(ptr: *mut c_void);
}
