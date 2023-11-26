macro_rules! vec_of_strings {
    ($($x:expr),*) => (vec![$(String::from($x)),*]);
}

fn main() {
    let mut lukes_loves: Vec<String> = vec_of_strings!["Nona", "Samantha", "Lucy", "Charles"];
    lukes_loves.push("myself".to_string());
    println!("I love {}!", test_project::format_list(lukes_loves));
}
