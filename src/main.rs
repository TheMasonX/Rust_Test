macro_rules! vec_of_strings {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

fn main() {
    let mut lukes_loves: Vec<String> = vec_of_strings!["Nona", "Samantha", "Lucy", "Charles"];
    lukes_loves.push("Myself".to_string());
    print_my_love(lukes_loves);
}

fn print_my_love(my_loves: Vec<String>) {
    let string_val: String = my_loves.join(", ");
    println!("I love {}!", string_val);
}
