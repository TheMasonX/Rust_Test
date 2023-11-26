
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
/// let path = "./run_file_test.txt";
/// let content = "input_string";
/// run_file_test(
///     |f| {
///         assert_eq!(
///             read_string(BufReader::new(f)).unwrap(),
///             content
///         );
///     },
///     path, content,
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
