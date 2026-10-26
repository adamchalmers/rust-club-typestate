/// Typesafe builder.
/// Users can only call `set_extension` once.
/// Same with `set_filename`.
/// They can call `read_only` as much or as little as they want (0, 1 or more times)
/// If you want to call `.build()` you MUST have called both `set_extension` and
/// `set_filename.`
///
/// There's two individual sub-states:
/// 1. Has filename been set (F)
/// 2. Has extension been set (E)
/// In both cases, unset is `()` (because there's no information to track)
/// and set means `String` (because there is information to track, specifically, what it was set to.)
pub struct FileBuilder<F, E> {
    pub filename: F,
    pub extension: E,
    pub read_only: bool,
}

/// Here `()` represents "has not been called".
/// There's two generics, both set to () because there's no data in either field.
impl Default for FileBuilder<(), ()> {
    fn default() -> Self {
        Self {
            filename: (),
            extension: (),
            read_only: false,
        }
    }
}

/// `<F, ()>` means the FileBuilder's state is generic over F (for filename),
/// i.e. it could be set or unset. But E is (), meaning extension is definitely not set.
///
/// It returns <F, String> meaning that whatever F was before, it's still that.
/// But the Extension generic has changed from () to String, meaning it's become set.
///
/// You can only call this method once, because calling it requires E = ()
/// but it returns something where E = String, so it no longer meets the condition to call it.
impl<F> FileBuilder<F, ()> {
    pub fn set_extension(self, extension: String) -> FileBuilder<F, String> {
        FileBuilder {
            filename: self.filename,
            extension,
            read_only: self.read_only,
        }
    }
}

/// Works just like the above, but this changes F instead of changing E.
impl<E> FileBuilder<(), E> {
    pub fn set_filename(self, filename: String) -> FileBuilder<String, E> {
        FileBuilder {
            filename,
            extension: self.extension,
            read_only: self.read_only,
        }
    }
}

/// Generic over both F and E, meaning they could be anything.
/// They could both be set, neither be set, or either one be set.
/// This doesn't change the type state (it takes FileBuilder<F, E> and also returns it, i.e. no change)
impl<F, E> FileBuilder<F, E> {
    pub fn read_only(self, read_only: bool) -> FileBuilder<F, E> {
        FileBuilder {
            extension: self.extension,
            filename: self.filename,
            read_only,
        }
    }
}

/// These methods are only defined when both F and E are String, i.e. both are set
/// (because neither of them are `()`, their unset state.)
impl FileBuilder<String, String> {
    /// ```compile_fail
    /// // This code is expected to fail compilation
    /// let builder = rust_club_typestate::FileBuilder::default()
    ///     .set_extension("kcl".to_owned())
    ///   .set_filename("kcl".to_owned());
    /// let filename = builder.build();
    /// ```
    pub fn build(self) -> String {
        format!("{}.{}", self.filename, self.extension)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_doesnt_matter() {
        // To start, the type is `FileBuilder<(), ()>`
        let builder = FileBuilder::default()
            // Type is now FileBuilder<(), ()>
            .set_extension("kcl".to_owned())
            // Type is now FileBuilder<(), String>
            .set_filename("main".to_owned())
            // Type is now FileBuilder<String, String>
            .read_only(true);
        // Only works when both extension and filename have been set.
        // Try commenting out `set_extension` above, this line should then
        // stop compiling.
        let filename = builder.build();
        assert_eq!(filename, "main.kcl")
    }
}
