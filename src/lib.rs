//! Mostly just an exploration of the Rust language and modern CI/CD practices, but I'll use it to include some common/useful utilities as I write them.

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
    ///            vec![]),
    ///            "nothing");
    /// ```
    ///
    /// ```
    /// # use tmx_utils::{vec_string, string_ext::format_list};
    /// assert_eq!(format_list(vec_string![
    ///            "Nona"]),
    ///            "Nona");
    /// ```
    ///
    /// ```
    /// # use tmx_utils::{vec_string, string_ext::format_list};
    /// assert_eq!(format_list(vec_string![
    ///            "Nona", "Samantha"]),
    ///            "Nona and Samantha");
    /// ```
    ///
    /// ```
    /// # use tmx_utils::{vec_string, string_ext::format_list};
    /// assert_eq!(format_list(vec_string![
    ///            "Nona", "Samantha", "Lucy", "Charles"]),
    ///            "Nona, Samantha, Lucy, and Charles");
    /// ```
    pub fn format_list(list: Vec<String>) -> String {
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
    ///            vec![]),
    ///            "nothing");
    /// ```
    ///
    /// ```
    /// # use tmx_utils::string_ext::format_list_slices;
    /// assert_eq!(format_list_slices(vec![
    ///            "Nona"]),
    ///            "Nona");
    /// ```
    ///
    /// ```
    /// # use tmx_utils::string_ext::format_list_slices;
    /// assert_eq!(format_list_slices(vec![
    ///            "Nona", "Samantha"]),
    ///            "Nona and Samantha");
    /// ```
    ///
    /// ```
    /// # use tmx_utils::string_ext::format_list_slices;
    /// assert_eq!(format_list_slices(vec![
    ///            "Nona", "Samantha", "Lucy", "Charles"]),
    ///            "Nona, Samantha, Lucy, and Charles");
    /// ```
    pub fn format_list_slices(list: Vec<&str>) -> String {
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

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_format_list() {
            assert_eq!(format_list(vec![]), "nothing");
            assert_eq!(format_list(vec!["Nona".to_string()]), "Nona");
            assert_eq!(
                format_list(vec!["Nona".to_string(), "Samantha".to_string()]),
                "Nona and Samantha"
            );
            assert_eq!(
                format_list(vec![
                    "Nona".to_string(),
                    "Samantha".to_string(),
                    "Lucy".to_string(),
                    "Charles".to_string()
                ]),
                "Nona, Samantha, Lucy, and Charles"
            );
        }

        #[test]
        fn test_format_list_slices() {
            assert_eq!(format_list_slices(vec![]), "nothing");
            assert_eq!(format_list_slices(vec!["Nona"]), "Nona");
            assert_eq!(
                format_list_slices(vec!["Nona", "Samantha"]),
                "Nona and Samantha"
            );
            assert_eq!(
                format_list_slices(vec!["Nona", "Samantha", "Lucy", "Charles"]),
                "Nona, Samantha, Lucy, and Charles"
            );
        }
    }
}
