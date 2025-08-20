/// File target struct.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileTarget {
    /// The name of the file target.
    ///
    /// By default, it is a empty string.
    pub name: String,
}

impl FileTarget {
    /// Create a new file target.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// use get_dir::FileTarget;
    ///
    /// let target: FileTarget = FileTarget::new("Cargo.toml");
    /// ```
    pub fn new<N: Into<String>>(name: N) -> Self {
        Self { name: name.into() }
    }
}

impl Default for FileTarget {
    fn default() -> Self {
        Self::new("")
    }
}
