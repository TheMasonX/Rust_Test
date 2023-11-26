/// Formats a list of strings with commas and "and" if necessary.
///
/// # Examples
///
/// ```
/// assert_eq!(test_project::format_list(
///            vec![]),
///            "nothing");
/// ```
///
/// ```
/// assert_eq!(test_project::format_list(
///            vec!["Nona".to_string()]),
///            "Nona");
/// ```
///
/// ```
/// assert_eq!(test_project::format_list(
///            vec!["Nona".to_string(),
///            "Samantha".to_string()]),
///            "Nona and Samantha");
/// ```
///
/// ```
/// assert_eq!(test_project::format_list(
///            vec!["Nona".to_string(),
///            "Samantha".to_string(),
///            "Lucy".to_string(),
///            "Charles".to_string()]),
///            "Nona, Samantha, Lucy, and Charles");
/// ```
///
pub fn format_list(list: Vec<String>) -> String {
    let love_count: usize = list.len();
    let string_val: String = match love_count {
        0 => String::from("nothing"),
        1 => list.concat().to_string(),
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
}
