pub mod old;
pub use old::FileBuilder;

/// This has two states:
/// Either it has an extension, or it doesn't.
pub struct File<Extension, const IS_UNDER_CWD: bool> {
    name: String,
    extension: Extension,
}

/// This works on either state.
impl<Extension, const IS_UNDER_CWD: bool> File<Extension, IS_UNDER_CWD> {
    pub fn name(&self) -> String {
        self.name.clone()
    }
}

pub struct HasExtension {
    extension: String,
}

/// Build a file whose state is "no extension".
impl File<(), false> {
    pub fn new(user_input: String) -> Self {
        Self {
            name: user_input,
            extension: (),
        }
    }
}

/// Has an extension, stored in a string.
impl File<HasExtension, true> {
    /// Build a file whose state is "yes extension".
    /// returns None if doesn't have an extension.
    pub fn new_with_extension_under_cwd(user_input: String) -> Option<Self> {
        let Some((filename, extension)) = user_input.rsplit_once('.') else {
            return None;
        };

        // Is this a relative path?
        // If so then obviously it's relative to the cwd

        let cwd = std::env::current_dir().unwrap().display().to_string();
        println!("cwd = {cwd}");
        println!("user_input = {user_input}");
        if let Some(_) = user_input.strip_prefix(&cwd) {
            Some(Self {
                name: filename.to_string(),
                extension: HasExtension {
                    extension: extension.to_string(),
                },
            })
        } else {
            None
        }
    }

    pub fn full_filepath(&self) -> String {
        format!("{}.{}", self.name, self.extension.extension)
    }

    /// Only works if the file has an extension.
    /// This does not return a runtime error if missing an extension, that is already
    /// proven to be true for all values of this type.
    pub fn open(&self) -> Result<String, String> {
        std::fs::read_to_string(&self.full_filepath()).map_err(|e| e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn can_get_name_with_or_without_extension() {
    //     let file0 = File::new("src/lib".to_owned());
    //     assert_eq!(file0.name(), "src/lib");
    //     let file1 = File::new_with_extension("src/lib.rs".to_owned()).unwrap();
    //     assert_eq!(file1.name(), "src/lib");
    // }

    #[test]
    fn test_filename_needs_extension() {
        let file = File::new_with_extension_under_cwd(
            "/Users/adamchalmers/programming/rust-club-typestate/src/lib.rs".to_owned(),
        )
        .unwrap();
        assert!(file.open().is_ok());
        assert!(
            File::new_with_extension_under_cwd(
                "/Users/adamchalmers/programming/rust-club-typestate/src/lib".to_owned()
            )
            .is_none()
        );
    }
}
