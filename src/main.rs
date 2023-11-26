use tmx_utils::string_ext;
use tmx_utils::vec_string;

fn main() {
    let mut lukes_loves: Vec<String> = vec_string!["Nona", "Samantha", "Lucy", "Charles"];
    lukes_loves.push("myself".to_string());
    println!("I love {}!", string_ext::format_list(lukes_loves));
}
