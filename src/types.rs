/// Diff algorithm used to align changes against the base text.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiffAlgorithm {
    /// Myers' general-purpose diff algorithm.
    Myers,
    /// Myers diff with extra work to minimize the edit script.
    Minimal,
    /// Patience diff, useful when unique lines provide strong anchors.
    Patience,
    /// Histogram diff, which extends patience diff to low-occurrence lines.
    Histogram,
}

/// Conflict-marker layout used in the merged output.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MergeStyle {
    /// Standard conflict markers containing the two variants.
    Normal,
    /// Diff3 markers that also include the common base.
    Diff3,
    /// Diff3 markers with Git's zealous conflict-boundary refinement.
    ZealousDiff3,
}

/// Automatic resolution applied to otherwise conflicting changes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MergeFavor {
    /// Resolve conflicts using the `ours` variant.
    Ours,
    /// Resolve conflicts using the `theirs` variant.
    Theirs,
    /// Resolve conflicts by including both variants.
    Union,
}

/// Aggressiveness used when refining overlapping changes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MergeLevel {
    /// Mark every overlapping change as a conflict.
    Minimal,
    /// Avoid conflicts when overlapping changes are identical.
    Eager,
    /// Refine non-identical changes to a smaller conflict set.
    Zealous,
    /// Apply zealous refinement while keeping punctuation-only gaps conflicted.
    ZealousAlnum,
}

/// Configuration for a three-way text merge.
#[derive(Debug, Clone)]
pub struct MergeOptions {
    /// Conflict-marker layout.
    pub style: MergeStyle,
    /// Optional automatic conflict resolution strategy.
    pub favor: Option<MergeFavor>,
    /// Diff algorithm used to align changes.
    pub algorithm: DiffAlgorithm,
    /// Number of repeated characters in each conflict marker.
    ///
    /// A value of zero is interpreted by xdiff as its default size of seven.
    pub marker_size: usize,
    /// Conflict refinement aggressiveness.
    pub level: MergeLevel,
    /// Optional label shown for the common base in diff3 output.
    pub base_label: Option<String>,
    /// Optional label shown for the `ours` variant.
    pub ours_label: Option<String>,
    /// Optional label shown for the `theirs` variant.
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

/// Owned merged text together with its unresolved conflict count.
#[derive(Debug, Clone)]
#[must_use]
pub struct MergeResult {
    /// Merged text, including conflict markers when conflicts remain.
    pub content: String,
    /// Number of unresolved conflict regions.
    pub conflicts: usize,
}

impl MergeResult {
    /// Returns true if there are any conflicts in the merge result.
    #[must_use]
    pub fn has_conflicts(&self) -> bool {
        self.conflicts > 0
    }

    /// Returns true if the merge was successful without conflicts.
    #[must_use]
    pub fn is_clean_merge(&self) -> bool {
        self.conflicts == 0
    }
}

/// Error returned when input validation or the native merge fails.
#[derive(Debug, thiserror::Error)]
pub enum MergeError {
    /// The native xdiff implementation violated its expected contract.
    #[error("Internal merge error: {0}")]
    Internal(String),
    /// An input cannot be represented safely by xdiff.
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    /// Native memory allocation failed.
    #[error("Memory allocation failed")]
    OutOfMemory,
}
