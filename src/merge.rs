use crate::ffi::{self, MmFile, MmBuffer, XmpParam, XppParam};
use crate::types::*;
use std::ffi::CString;
use std::os::raw::{c_char, c_int, c_long, c_ulong};
use std::ptr;


fn string_to_mmfile(s: &str) -> MmFile {
    MmFile {
        ptr: s.as_ptr() as *mut c_char,
        size: s.len() as c_long,
    }
}

fn count_conflicts(content: &str) -> usize {
    content.lines()
        .filter(|line| line.starts_with("<<<<<<<") || line.starts_with("=======") || line.starts_with(">>>>>>>"))
        .count() / 3
}

pub fn merge_strings(
    base: &str,
    ours: &str,
    theirs: &str,
    options: &MergeOptions,
) -> Result<MergeResult, MergeError> {
    // Keep CStrings alive for the entire function
    let ancestor_cstr = options.ancestor_label
        .as_ref()
        .map(|s| CString::new(s.as_str()))
        .transpose()
        .map_err(|_| MergeError::InvalidInput("Invalid ancestor label".to_string()))?;
    
    let file1_cstr = options.ours_label
        .as_ref()
        .map(|s| CString::new(s.as_str()))
        .transpose()
        .map_err(|_| MergeError::InvalidInput("Invalid ours label".to_string()))?;
    
    let file2_cstr = options.theirs_label
        .as_ref()
        .map(|s| CString::new(s.as_str()))
        .transpose()
        .map_err(|_| MergeError::InvalidInput("Invalid theirs label".to_string()))?;

    let mut flags = 0 as c_ulong;
    match options.algorithm {
        DiffAlgorithm::Myers => {},
        DiffAlgorithm::Minimal => flags |= ffi::XDF_NEED_MINIMAL,
        DiffAlgorithm::Patience => flags |= ffi::XDF_PATIENCE_DIFF,
        DiffAlgorithm::Histogram => flags |= ffi::XDF_HISTOGRAM_DIFF,
    }

    let level = match options.level {
        MergeLevel::Minimal => ffi::XDL_MERGE_MINIMAL,
        MergeLevel::Eager => ffi::XDL_MERGE_EAGER,
        MergeLevel::Zealous => ffi::XDL_MERGE_ZEALOUS,
        MergeLevel::ZealousAlnum => ffi::XDL_MERGE_ZEALOUS_ALNUM,
    };

    let favor = match options.favor {
        None => 0,
        Some(MergeFavor::Ours) => ffi::XDL_MERGE_FAVOR_OURS,
        Some(MergeFavor::Theirs) => ffi::XDL_MERGE_FAVOR_THEIRS,
        Some(MergeFavor::Union) => ffi::XDL_MERGE_FAVOR_UNION,
    };

    let style = match options.style {
        MergeStyle::Normal => 0,
        MergeStyle::Diff3 => ffi::XDL_MERGE_DIFF3,
        MergeStyle::ZealousDiff3 => ffi::XDL_MERGE_ZEALOUS_DIFF3,
    };

    let xmp = XmpParam {
        xpp: XppParam {
            flags,
            ignore_regex: ptr::null_mut(),
            ignore_regex_nr: 0,
            anchors: ptr::null_mut(),
            anchors_nr: 0,
        },
        marker_size: options.marker_size as c_int,
        level,
        favor,
        style,
        ancestor: ancestor_cstr.as_ref().map_or(ptr::null(), |s| s.as_ptr()),
        file1: file1_cstr.as_ref().map_or(ptr::null(), |s| s.as_ptr()),
        file2: file2_cstr.as_ref().map_or(ptr::null(), |s| s.as_ptr()),
    };
    
    let base_mmf = string_to_mmfile(base);
    let ours_mmf = string_to_mmfile(ours);
    let theirs_mmf = string_to_mmfile(theirs);
    
    let mut result = MmBuffer {
        ptr: ptr::null_mut(),
        size: 0,
    };

    let ret = unsafe {
        ffi::xdl_merge(&base_mmf, &ours_mmf, &theirs_mmf, &xmp, &mut result)
    };

    if ret < 0 {
        return Err(MergeError::Internal(format!("xdl_merge failed with code {}", ret)));
    }

    if result.ptr.is_null() {
        return Ok(MergeResult {
            content: String::new(),
            conflicts: 0,
        });
    }

    let content = unsafe {
        let slice = std::slice::from_raw_parts(result.ptr as *const u8, result.size as usize);
        String::from_utf8_lossy(slice).into_owned()
    };

    let conflicts = count_conflicts(&content);

    // Free the memory allocated by xdiff
    unsafe {
        libc::free(result.ptr as *mut libc::c_void);
    }

    Ok(MergeResult {
        content,
        conflicts,
    })
}