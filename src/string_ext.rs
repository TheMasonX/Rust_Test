use itertools::Itertools;

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
// pub fn format_list(list: &[String]) -> String {
//     format_list_slices(&list.iter().map(|s| s as &str).collect_vec())
// }

pub fn format_list(list: &[String]) -> String {
    format_list_slices(&list.iter().map(|s| s as &str).collect_vec())
}

/// Formats a list of string slices with commas and "and" if necessary. Uses the oxford comma.<br>
/// <i>Note: An attempt was made to genericize these functions to work with Vec<T : Display + Join>, but the Join trait is unstable.</i>
///
/// # Examples
///
/// ```
/// # use tmx_utils::string_ext::format_list_slices;
/// assert_eq!(format_list_slices(
///            &[]),
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
/// assert_eq!(format_list_slices(&[
///            "Nona", "Samantha"]),
///            "Nona and Samantha");
/// ```
///
/// ```
/// # use tmx_utils::string_ext::format_list_slices;
/// assert_eq!(format_list_slices(&[
///            "Nona", "Samantha", "Lucy", "Charles"]),
///            "Nona, Samantha, Lucy, and Charles");
/// ```
pub fn format_list_slices(list: &[&str]) -> String {
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
/// let path = "./read_string.txt";
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

/// Reads a file from the current directory. Returns io errors or an error if the line is empty.
///
/// # Examples
///
/// ```
/// # use tmx_utils::string_ext::read_local_file;
/// let e = read_local_file("read_local_file.txt").unwrap_err();
/// println!("{}", e.kind());
/// assert_eq!(e.kind(), std::io::ErrorKind::NotFound); //Not a valid path
/// ```
///
/// ```
/// # use tmx_utils::string_ext::read_local_file;
/// let e = read_local_file("src/main.rs").unwrap();
/// assert!(e.len() > 0);
/// ```
pub fn read_local_file(path: &str) -> Result<String, std::io::Error> {
    let input_file = format!(
        "{}/{}",
        std::env::current_dir()?.to_str().unwrap_or_default(),
        path
    );
    std::fs::read_to_string(input_file)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_list() {
        assert_eq!(format_list(&[]), "nothing");
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
        assert_eq!(format_list_slices(&[]), "nothing");
        assert_eq!(format_list_slices(&["Nona"]), "Nona");
        assert_eq!(
            format_list_slices(&["Nona", "Samantha"]),
            "Nona and Samantha"
        );
        assert_eq!(
            format_list_slices(&["Nona", "Samantha", "Lucy", "Charles"]),
            "Nona, Samantha, Lucy, and Charles"
        );
    }

    #[test]
    fn test_read_input() {
        use super::read_string;
        let path = "./test_read_input.txt";
        let content = "input_string";
        crate::test_utils::run_file_test(
            |f| {
                assert_eq!(read_string(std::io::BufReader::new(f)).unwrap(), content);
            },
            path,
            content,
        )
    }
}
