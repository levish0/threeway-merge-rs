use crate::ffi;
use crate::types::*;
use std::ffi::CString;
use std::os::raw::{c_char, c_int, c_long, c_void};
use std::ptr;

// xdiff accumulates merge output sizes in a signed C int. Inputs are rejected
// before FFI whenever a conservative output bound could exceed that range.
const MAX_XDIFF_OUTPUT_SIZE: usize = c_int::MAX as usize;
const DEFAULT_MARKER_SIZE: usize = 7;

struct XdiffBuffer {
    ptr: *mut c_char,
}

impl Drop for XdiffBuffer {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            // SAFETY: threeway_merge_run returns this pointer from xdl_malloc,
            // and the paired shim function releases it with xdl_free.
            unsafe { ffi::threeway_merge_free(self.ptr.cast::<c_void>()) };
        }
    }
}

fn checked_add(left: usize, right: usize, field: &str) -> Result<usize, MergeError> {
    left.checked_add(right).ok_or_else(|| {
        MergeError::InvalidInput(format!("{field} exceeds xdiff's supported size range"))
    })
}

fn checked_mul(left: usize, right: usize, field: &str) -> Result<usize, MergeError> {
    left.checked_mul(right).ok_or_else(|| {
        MergeError::InvalidInput(format!("{field} exceeds xdiff's supported size range"))
    })
}

fn line_count(input: &str) -> usize {
    if input.is_empty() {
        return 0;
    }

    input.bytes().filter(|&byte| byte == b'\n').count() + usize::from(!input.ends_with('\n'))
}

fn label_output_size(label: Option<&String>, field: &str) -> Result<usize, MergeError> {
    let Some(label) = label else {
        return Ok(0);
    };

    if label.as_bytes().contains(&0) {
        return Err(MergeError::InvalidInput(format!(
            "{field} label contains a NUL byte"
        )));
    }

    // xdiff stores strlen(label) + one separating space in a signed C int.
    let size = checked_add(label.len(), 1, field)?;
    if size > MAX_XDIFF_OUTPUT_SIZE {
        return Err(MergeError::InvalidInput(format!(
            "{field} label exceeds xdiff's supported size range"
        )));
    }
    Ok(size)
}

fn validate_output_bound(
    base: &str,
    ours: &str,
    theirs: &str,
    options: &MergeOptions,
    base_label_size: usize,
    ours_label_size: usize,
    theirs_label_size: usize,
) -> Result<(), MergeError> {
    let input_size = checked_add(base.len(), ours.len(), "merge input")?;
    let input_size = checked_add(input_size, theirs.len(), "merge input")?;

    let conflict_bound = checked_add(line_count(base), line_count(ours), "line count")?;
    let conflict_bound = checked_add(conflict_bound, line_count(theirs), "line count")?;

    let marker_size = if options.marker_size == 0 {
        DEFAULT_MARKER_SIZE
    } else {
        options.marker_size
    };
    let marker_count = match options.style {
        MergeStyle::Normal => 3,
        MergeStyle::Diff3 | MergeStyle::ZealousDiff3 => 4,
    };

    let marker_bytes = checked_mul(marker_size, marker_count, "conflict markers")?;
    let mut label_bytes = checked_add(ours_label_size, theirs_label_size, "labels")?;
    if !matches!(options.style, MergeStyle::Normal) {
        label_bytes = checked_add(label_bytes, base_label_size, "labels")?;
    }

    // Every marker may use CRLF, and xdiff may append CRLF to each of the
    // three conflict sections when its source line lacks a terminator.
    let marker_line_endings = checked_mul(marker_count, 2, "conflict markers")?;
    let per_conflict = checked_add(marker_bytes, label_bytes, "merge output")?;
    let per_conflict = checked_add(per_conflict, marker_line_endings, "merge output")?;
    let per_conflict = checked_add(per_conflict, 6, "merge output")?;
    let conflict_overhead = checked_mul(conflict_bound, per_conflict, "merge output")?;
    let output_bound = checked_add(input_size, conflict_overhead, "merge output")?;

    if output_bound > MAX_XDIFF_OUTPUT_SIZE {
        return Err(MergeError::InvalidInput(format!(
            "merge output may exceed xdiff's supported size limit ({MAX_XDIFF_OUTPUT_SIZE} bytes)"
        )));
    }

    Ok(())
}

fn string_size(input: &str, field: &str) -> Result<c_long, MergeError> {
    c_long::try_from(input.len()).map_err(|_| {
        MergeError::InvalidInput(format!(
            "{field} is too large for xdiff input size ({})",
            input.len()
        ))
    })
}

fn label_to_cstring(label: Option<&String>) -> Result<Option<CString>, MergeError> {
    label
        .map(|label| CString::new(label.as_str()))
        .transpose()
        .map_err(|_| MergeError::Internal("validated label contained a NUL byte".to_string()))
}

/// Merges two text variants relative to a common base.
///
/// The returned content always owns its data. A successful result may still
/// contain conflicts; inspect [`MergeResult::has_conflicts`] or
/// [`MergeResult::conflicts`] to distinguish a clean merge.
pub fn merge_strings(
    base: &str,
    ours: &str,
    theirs: &str,
    options: &MergeOptions,
) -> Result<MergeResult, MergeError> {
    let base_label_size = label_output_size(options.base_label.as_ref(), "base")?;
    let ours_label_size = label_output_size(options.ours_label.as_ref(), "ours")?;
    let theirs_label_size = label_output_size(options.theirs_label.as_ref(), "theirs")?;

    let marker_size = c_int::try_from(options.marker_size).map_err(|_| {
        MergeError::InvalidInput(format!(
            "marker_size ({}) exceeds supported range",
            options.marker_size
        ))
    })?;

    // Fast paths intentionally follow option validation, preserving the API's
    // behavior for invalid labels and marker sizes.
    if ours == theirs {
        return Ok(MergeResult {
            content: ours.to_owned(),
            conflicts: 0,
        });
    }
    if ours == base {
        return Ok(MergeResult {
            content: theirs.to_owned(),
            conflicts: 0,
        });
    }
    if theirs == base {
        return Ok(MergeResult {
            content: ours.to_owned(),
            conflicts: 0,
        });
    }

    validate_output_bound(
        base,
        ours,
        theirs,
        options,
        base_label_size,
        ours_label_size,
        theirs_label_size,
    )?;

    let base_cstr = label_to_cstring(options.base_label.as_ref())?;
    let ours_cstr = label_to_cstring(options.ours_label.as_ref())?;
    let theirs_cstr = label_to_cstring(options.theirs_label.as_ref())?;

    let algorithm = match options.algorithm {
        DiffAlgorithm::Myers => 0,
        DiffAlgorithm::Minimal => 1,
        DiffAlgorithm::Patience => 2,
        DiffAlgorithm::Histogram => 3,
    };
    let level = match options.level {
        MergeLevel::Minimal => 0,
        MergeLevel::Eager => 1,
        MergeLevel::Zealous => 2,
        MergeLevel::ZealousAlnum => 3,
    };
    let favor = match options.favor {
        None => 0,
        Some(MergeFavor::Ours) => 1,
        Some(MergeFavor::Theirs) => 2,
        Some(MergeFavor::Union) => 3,
    };
    let style = match options.style {
        MergeStyle::Normal => 0,
        MergeStyle::Diff3 => 1,
        MergeStyle::ZealousDiff3 => 2,
    };

    let mut result_ptr = ptr::null_mut();
    let mut result_size = 0;

    // SAFETY: all input pointers remain valid for the call, their checked
    // lengths match the referenced strings, labels are live CStrings, and the
    // two output pointers refer to writable local variables. The shim treats
    // input buffers as read-only despite xdiff's historical mutable typedef.
    let ret = unsafe {
        ffi::threeway_merge_run(
            base.as_ptr().cast::<c_char>(),
            string_size(base, "base")?,
            ours.as_ptr().cast::<c_char>(),
            string_size(ours, "ours")?,
            theirs.as_ptr().cast::<c_char>(),
            string_size(theirs, "theirs")?,
            algorithm,
            marker_size,
            level,
            favor,
            style,
            base_cstr
                .as_ref()
                .map_or(ptr::null(), |label| label.as_ptr()),
            ours_cstr
                .as_ref()
                .map_or(ptr::null(), |label| label.as_ptr()),
            theirs_cstr
                .as_ref()
                .map_or(ptr::null(), |label| label.as_ptr()),
            &mut result_ptr,
            &mut result_size,
        )
    };
    let result = XdiffBuffer { ptr: result_ptr };

    if ret < 0 {
        return Err(MergeError::Internal(format!(
            "xdl_merge failed with code {ret}"
        )));
    }

    if result.ptr.is_null() {
        if result_size == 0 {
            return Ok(MergeResult {
                content: String::new(),
                conflicts: ret as usize,
            });
        }
        return Err(MergeError::Internal(format!(
            "xdl_merge returned null buffer with non-zero size ({result_size})"
        )));
    }

    let result_size = usize::try_from(result_size).map_err(|_| {
        MergeError::Internal(format!(
            "xdl_merge returned invalid output size ({result_size})"
        ))
    })?;
    if result_size > MAX_XDIFF_OUTPUT_SIZE {
        return Err(MergeError::Internal(format!(
            "xdl_merge returned oversized output ({result_size})"
        )));
    }

    // SAFETY: the shim returned a non-null xdl_malloc buffer of result_size
    // bytes. Xdiff output is composed only from valid UTF-8 input/labels and
    // ASCII conflict markers, so invalid UTF-8 indicates an FFI invariant bug.
    let bytes = unsafe { std::slice::from_raw_parts(result.ptr.cast::<u8>(), result_size) };
    let content = String::from_utf8(bytes.to_vec())
        .map_err(|_| MergeError::Internal("xdl_merge returned invalid UTF-8 output".to_string()))?;

    Ok(MergeResult {
        content,
        conflicts: ret as usize,
    })
}
