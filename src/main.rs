macro_rules! vec_of_strings {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

// macro_rules! unwrap_or_unknown {
//     ($($x:expr)) => {
//         $x.unwrap_or(String::from("Unknown"))
//     };
// }

fn main() {
    let mut lukes_loves: Vec<String> = vec_of_strings!["Nona", "Samantha", "Lucy", "Charles"];
    lukes_loves.push("myself".to_string());
    print_my_love(lukes_loves);
}

fn print_my_love(my_loves: Vec<String>) {
    let string_val: String = match my_loves.len() {
        0 => "nobody".to_string(),
        1 => my_loves.concat().to_string(),
        2 => format!(
            "{} and {}",
            my_loves.first().unwrap(),
            my_loves.last().unwrap(),
        ),
        _ => format!(
            "{}, and {}",
            my_loves[..my_loves.len() - 1].join(", "),
            my_loves.last().unwrap(),
        ),
    };
    println!("I love {}!", string_val);
}
