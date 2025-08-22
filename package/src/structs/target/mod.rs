pub mod dir;

pub mod file;

use crate::structs::target::{dir::DirTarget, file::FileTarget};

/// Enum to determine whether the target is a directory or a file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Target {
    /// The target is a directory.
    Dir(DirTarget),
    /// The target is a file.
    File(FileTarget),
}
