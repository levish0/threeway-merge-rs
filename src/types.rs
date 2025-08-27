#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiffAlgorithm {
    Myers,
    Minimal,
    Patience,
    Histogram,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MergeStyle {
    Normal,
    Diff3,
    ZealousDiff3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MergeFavor {
    Ours,
    Theirs,
    Union,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MergeLevel {
    Minimal,
    Eager,
    Zealous,
    ZealousAlnum,
}

#[derive(Debug, Clone)]
pub struct MergeOptions {
    pub style: MergeStyle,
    pub favor: Option<MergeFavor>,
    pub algorithm: DiffAlgorithm,
    pub marker_size: usize,
    pub level: MergeLevel,
    pub base_label: Option<String>,
    pub ours_label: Option<String>,
    pub theirs_label: Option<String>,
}

impl Default for MergeOptions {
    fn default() -> Self {
        Self {
            style: MergeStyle::Normal,
            favor: None,
            algorithm: DiffAlgorithm::Myers,
            marker_size: 7,
            level: MergeLevel::ZealousAlnum,
            base_label: None,
            ours_label: None,
            theirs_label: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MergeResult {
    pub content: String,
    pub conflicts: usize,
}

impl MergeResult {
    /// Returns true if there are any conflicts in the merge result
    pub fn has_conflicts(&self) -> bool {
        self.conflicts > 0
    }

    /// Returns true if the merge was successful without conflicts
    pub fn is_clean_merge(&self) -> bool {
        self.conflicts == 0
    }
}

#[derive(Debug, thiserror::Error)]
pub enum MergeError {
    #[error("Internal merge error: {0}")]
    Internal(String),
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Memory allocation failed")]
    OutOfMemory,
}
