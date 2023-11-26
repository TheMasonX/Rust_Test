//! Mostly just an exploration of the Rust language and modern CI/CD practices, but I'll use it to include some common/useful utilities as I write them.

/// Testing utilities
pub mod test_utils {

    /// Runs a test with a teardown function that is guaranteed to run
    pub fn run_test<T, U>(test: T, teardown: U)
    where
        T: FnOnce() + std::panic::UnwindSafe,
        U: FnOnce(),
    {
        let result = std::panic::catch_unwind(test);

        teardown();

        assert!(result.is_ok())
    }

    /// Runs a test on a File with a teardown function that is guaranteed to delete the file after the test
    ///
    /// # Panics
    ///
    /// If the file cannot be opened
    ///
    /// # Examples
    ///
    /// ```
    /// # use tmx_utils::test_utils::run_file_test;
    /// use tmx_utils::string_ext::read_string;
    /// use std::io::BufReader;
    ///
    /// run_file_test(
    ///     |f| {
    ///         assert_eq!(
    ///             read_string(BufReader::new(f)).unwrap(),
    ///             "input_string"
    ///         );
    ///     },
    ///     "./run_file_test.txt", "input_string",
    /// );
    /// ```
    pub fn run_file_test<T>(test: T, path: &str, contents: &str)
    where
        T: FnOnce(std::fs::File) + std::panic::UnwindSafe,
    {
        let teardown = || {
            std::fs::remove_file(path).unwrap();
        };

        std::fs::write(path, contents).unwrap();
        let file = match std::fs::File::open(path) {
            Ok(f) => f,
            Err(e) => {
                teardown();
                panic!(
                    "Couldn't open {} after writing, deleting. Error: {:?}",
                    path, e
                )
            }
        };
        run_test(|| test(file), teardown);
    }

    #[cfg(test)]
    mod tests {
        use crate::string_ext;

        use super::*;

        #[test]
        fn test_run_file_test() {
            run_file_test(
                |f| {
                    assert_eq!(
                        string_ext::read_string(std::io::BufReader::new(f)).unwrap(),
                        "input_string"
                    );
                },
                "./test.txt",
                "input_string",
            );
        }
    }
}

/// String Extensions
pub mod string_ext {
    /// Declares a `Vec<String>` and casts string literals from `&str` to `String`
    ///
    /// # Examples
    ///
    /// ```
    /// assert_eq!(tmx_utils::vec_string!["I was once a string slice &str"],
    ///            vec!["I was once a string slice &str".to_string()]);
    /// ```
    #[macro_export]
    macro_rules! vec_string {
    ($($x:expr),*) => (vec![$(String::from($x)),*]);
}

    /// Formats a list of strings with commas and "and" if necessary. Uses the oxford comma.<br>
    /// <i>Note: An attempt was made to genericize these functions to work with Vec<T : Display + Join>, but the Join trait is unstable.</i>
    ///
    /// # Examples
    ///
    /// ```
    /// # use tmx_utils::string_ext::format_list;
    /// assert_eq!(format_list(
    ///            &vec![]),
    ///            "nothing");
    /// ```
    ///
    /// ```
    /// # use tmx_utils::{vec_string, string_ext::format_list};
    /// assert_eq!(format_list(&vec_string![
    ///            "Nona"]),
    ///            "Nona");
    /// ```
    ///
    /// ```
    /// # use tmx_utils::{vec_string, string_ext::format_list};
    /// assert_eq!(format_list(&vec_string![
    ///            "Nona", "Samantha"]),
    ///            "Nona and Samantha");
    /// ```
    ///
    /// ```
    /// # use tmx_utils::{vec_string, string_ext::format_list};
    /// assert_eq!(format_list(&vec_string![
    ///            "Nona", "Samantha", "Lucy", "Charles"]),
    ///            "Nona, Samantha, Lucy, and Charles");
    /// ```
    pub fn format_list(list: &Vec<String>) -> String {
        let love_count: usize = list.len();
        let string_val: String = match love_count {
            0 => String::from("nothing"),
            1 => list[0].to_string(),
            2 => format!("{} and {}", list[0], list[1]),
            _ => format!(
                "{}, and {}",
                list[..love_count - 1].join(", "),
                list[love_count - 1],
            ),
        };
        string_val
    }

    /// Formats a list of string slices with commas and "and" if necessary. Uses the oxford comma.<br>
    /// <i>Note: An attempt was made to genericize these functions to work with Vec<T : Display + Join>, but the Join trait is unstable.</i>
    ///
    /// # Examples
    ///
    /// ```
    /// # use tmx_utils::string_ext::format_list_slices;
    /// assert_eq!(format_list_slices(
    ///            &vec![]),
    ///            "nothing");
    /// ```
    ///
    /// ```
    /// # use tmx_utils::string_ext::format_list_slices;
    /// assert_eq!(format_list_slices(&vec![
    ///            "Nona"]),
    ///            "Nona");
    /// ```
    ///
    /// ```
    /// # use tmx_utils::string_ext::format_list_slices;
    /// assert_eq!(format_list_slices(&vec![
    ///            "Nona", "Samantha"]),
    ///            "Nona and Samantha");
    /// ```
    ///
    /// ```
    /// # use tmx_utils::string_ext::format_list_slices;
    /// assert_eq!(format_list_slices(&vec![
    ///            "Nona", "Samantha", "Lucy", "Charles"]),
    ///            "Nona, Samantha, Lucy, and Charles");
    /// ```
    pub fn format_list_slices(list: &Vec<&str>) -> String {
        let love_count: usize = list.len();
        let string_val: String = match love_count {
            0 => String::from("nothing"),
            1 => list[0].to_string(),
            2 => format!("{} and {}", list[0], list[1]),
            _ => format!(
                "{}, and {}",
                list[..love_count - 1].join(", "),
                list[love_count - 1],
            ),
        };
        string_val
    }

    /// Reads a line from `std::io::stdin().lock()` and trims it. Returns an error if the line is empty or if an error is returned from `read_line()`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tmx_utils::string_ext::read_string_stdin;
    /// let e = read_string_stdin().err().unwrap();
    /// assert!(e.kind() == std::io::ErrorKind::InvalidInput);
    /// ```
    pub fn read_string_stdin() -> Result<String, std::io::Error> {
        read_string(&mut std::io::stdin().lock())
    }

    /// Reads a line from a reader and trims it. Returns io errors or an error if the line is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tmx_utils::string_ext::read_string;
    /// let e = read_string(&mut std::io::stdin().lock()).err().unwrap();
    /// assert!(e.kind() == std::io::ErrorKind::InvalidInput);
    /// ```
    ///
    /// ```
    /// # use tmx_utils::string_ext::read_string;
    /// use tmx_utils::test_utils::run_file_test;
    /// use std::io::BufReader;
    ///
    /// run_file_test(
    ///     |f| {
    ///         assert_eq!(
    ///             read_string(BufReader::new(f)).unwrap(),
    ///             "input_string"
    ///         );
    ///     },
    ///     "./read_string.txt", "input_string",
    /// );
    /// ```
    pub fn read_string<R>(mut reader: R) -> Result<String, std::io::Error>
    where
        R: std::io::BufRead,
    {
        let mut input = String::new();
        reader.read_line(&mut input)?;
        let trimmed = input.trim();
        match trimmed.len() {
            0 => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Input was empty",
            )),
            _ => Ok(trimmed.to_string()),
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_format_list() {
            assert_eq!(format_list(&vec![]), "nothing");
            assert_eq!(format_list(&vec_string!["Nona"]), "Nona");
            assert_eq!(
                format_list(&vec_string!["Nona", "Samantha"]),
                "Nona and Samantha"
            );
            assert_eq!(
                format_list(&vec_string!["Nona", "Samantha", "Lucy", "Charles"]),
                "Nona, Samantha, Lucy, and Charles"
            );
        }

        #[test]
        fn test_format_list_slices() {
            assert_eq!(format_list_slices(&vec![]), "nothing");
            assert_eq!(format_list_slices(&vec!["Nona"]), "Nona");
            assert_eq!(
                format_list_slices(&vec!["Nona", "Samantha"]),
                "Nona and Samantha"
            );
            assert_eq!(
                format_list_slices(&vec!["Nona", "Samantha", "Lucy", "Charles"]),
                "Nona, Samantha, Lucy, and Charles"
            );
        }

        #[test]
        fn test_read_input() {
            use super::read_string;
            use std::fs::File;
            let path = "./test_read_input.txt";
            std::fs::write(path, "input_string").unwrap();
            let file = match File::open(path) {
                Ok(f) => f,
                Err(e) => {
                    std::fs::remove_file(path).unwrap();
                    panic!("Couldn't open {}, deleting: {:?}", path, e)
                }
            };
            let result = read_string(&mut std::io::BufReader::new(file));
            std::fs::remove_file(path).unwrap(); //Clean up
            assert_eq!(result.unwrap(), "input_string");
        }
    }
}
