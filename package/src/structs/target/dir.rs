/// Directory target struct.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirTarget {
    /// The name of the directory target.
    ///
    /// By default, it is a empty string.
    pub name: String,
}

impl DirTarget {
    /// Create a new directory target.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// use get_dir::DirTarget;
    ///
    /// let target: DirTarget = DirTarget::new("src");
    /// ```
    pub fn new<N: Into<String>>(name: N) -> Self {
        Self { name: name.into() }
    }
}

impl Default for DirTarget {
    fn default() -> Self {
        Self::new("")
    }
}
