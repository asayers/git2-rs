use std::marker;
use std::mem;
use libc::c_uint;

use {raw, Object};
use util::Binding;

/// annotated commits, the input to merge and rebase
pub struct AnnotatedCommit<'repo> {
    raw: *mut raw::git_annotated_commit,
    _marker: marker::PhantomData<Object<'repo>>,
}

/// merge options
// modeled after DiffFindOptions
pub struct MergeOptions {
    raw: raw::git_merge_options,
}

impl MergeOptions {
    /// Creates a default set of merge options.
    pub fn new() -> MergeOptions {
        let mut opts = MergeOptions {
            raw: unsafe { mem::zeroed() },
        };
        assert_eq!(unsafe {
            raw::git_merge_init_options(&mut opts.raw, 1)
        }, 0);
        opts
    }

    /// Similarity to consider a file renamed (default 50)
    pub fn rename_threshold(&mut self, thresh: u32) -> &mut MergeOptions {
        self.raw.rename_threshold = thresh;
        self
    }

    ///  Maximum similarity sources to examine for renames (default 200).
    /// If the number of rename candidates (add / delete pairs) is greater than this value,
    /// inexact rename detection is aborted. This setting overrides the `merge.renameLimit`
    /// configuration value.
    pub fn target_limit(&mut self, limit: u32) -> &mut MergeOptions {
        self.raw.target_limit = limit as c_uint;
        self
    }

    fn flag(&mut self, opt: u32, val: bool) -> &mut MergeOptions {
        if val {
            self.raw.file_flags |= opt;
        } else {
            self.raw.file_flags &= !opt;
        }
        self
    }

    /// Create standard conflicted merge files
    pub fn standard_style(&mut self, find: bool) -> &mut MergeOptions {
        self.flag(raw::GIT_MERGE_FILE_STYLE_MERGE, find)
    }

    /// Create diff3-style file
    pub fn diff3_style(&mut self, find: bool) -> &mut MergeOptions {
        self.flag(raw::GIT_MERGE_FILE_STYLE_DIFF3, find)
    }

    /// Condense non-alphanumeric regions for simplified diff file
    pub fn simplify_alnum(&mut self, find: bool) -> &mut MergeOptions {
        self.flag(raw::GIT_MERGE_FILE_SIMPLIFY_ALNUM, find)
    }

    /// Ignore all whitespace
    pub fn ignore_whitespace(&mut self, find: bool) -> &mut MergeOptions {
        self.flag(raw::GIT_MERGE_FILE_IGNORE_WHITESPACE, find)
    }

    /// Ignore changes in amount of whitespace
    pub fn ignore_whitespace_change(&mut self, find: bool) -> &mut MergeOptions {
        self.flag(raw::GIT_MERGE_FILE_IGNORE_WHITESPACE_CHANGE, find)
    }

    /// Ignore whitespace at end of line
    pub fn ignore_whitespace_eol(&mut self, find: bool) -> &mut MergeOptions {
        self.flag(raw::GIT_MERGE_FILE_IGNORE_WHITESPACE_EOL, find)
    }

    /// Use the "patience diff" algorithm
    pub fn patience(&mut self, find: bool) -> &mut MergeOptions {
        self.flag(raw::GIT_MERGE_FILE_DIFF_PATIENCE, find)
    }

    /// Take extra time to find minimal diff
    pub fn minimal(&mut self, find: bool) -> &mut MergeOptions {
        self.flag(raw::GIT_MERGE_FILE_DIFF_MINIMAL, find)
    }

    /// Acquire a pointer to the underlying raw options.
    ///
    /// This function is unsafe as the pointer is only valid so long as this
    /// structure is not moved, modified, or used elsewhere.
    // modeled after DiffOptions.raw()
    pub unsafe fn raw(&self) -> *const raw::git_merge_options {
        &self.raw as *const _
    }
}

impl<'repo> Binding for AnnotatedCommit<'repo> {
    type Raw = *mut raw::git_annotated_commit;
    unsafe fn from_raw(raw: *mut raw::git_annotated_commit) -> AnnotatedCommit<'repo> {
        AnnotatedCommit {
            raw: raw,
            _marker: marker::PhantomData,
        }
    }
    fn raw(&self) -> *mut raw::git_annotated_commit { self.raw }
}

impl<'repo> Drop for AnnotatedCommit<'repo> {
    fn drop(&mut self) {
        unsafe { raw::git_annotated_commit_free(self.raw) }
    }
}
