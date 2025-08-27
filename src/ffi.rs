use std::os::raw::{c_char, c_int, c_long, c_ulong, c_void};

// FFI bindings for libgit2/xdiff

#[repr(C)]
pub struct MmFile {
    pub ptr: *mut c_char,
    pub size: c_long,
}

#[repr(C)]
pub struct MmBuffer {
    pub ptr: *mut c_char,
    pub size: c_long,
}

#[repr(C)]
pub struct XppParam {
    pub flags: c_ulong,
    pub ignore_regex: *mut *mut c_void, // xdl_regex_t**
    pub ignore_regex_nr: usize,
    pub anchors: *mut *mut c_char,
    pub anchors_nr: usize,
}

#[repr(C)]
pub struct XmpParam {
    pub xpp: XppParam,
    pub marker_size: c_int,
    pub level: c_int,
    pub favor: c_int,
    pub style: c_int,
    pub ancestor: *const c_char,
    pub file1: *const c_char,
    pub file2: *const c_char,
}

// xdiff constants from xdiff.h
pub const XDF_NEED_MINIMAL: c_ulong = 1 << 0;
pub const XDF_PATIENCE_DIFF: c_ulong = 1 << 14;
pub const XDF_HISTOGRAM_DIFF: c_ulong = 1 << 15;
pub const XDF_DIFF_ALGORITHM_MASK: c_ulong = XDF_PATIENCE_DIFF | XDF_HISTOGRAM_DIFF;

pub const XDL_MERGE_MINIMAL: c_int = 0;
pub const XDL_MERGE_EAGER: c_int = 1;
pub const XDL_MERGE_ZEALOUS: c_int = 2;
pub const XDL_MERGE_ZEALOUS_ALNUM: c_int = 3;

pub const XDL_MERGE_FAVOR_OURS: c_int = 1;
pub const XDL_MERGE_FAVOR_THEIRS: c_int = 2;
pub const XDL_MERGE_FAVOR_UNION: c_int = 3;

pub const XDL_MERGE_DIFF3: c_int = 1;
pub const XDL_MERGE_ZEALOUS_DIFF3: c_int = 2;

pub const DEFAULT_CONFLICT_MARKER_SIZE: c_int = 7;

unsafe extern "C" {
    pub fn xdl_merge(
        orig: *const MmFile,
        mf1: *const MmFile,
        mf2: *const MmFile,
        xmp: *const XmpParam,
        result: *mut MmBuffer,
    ) -> c_int;
}