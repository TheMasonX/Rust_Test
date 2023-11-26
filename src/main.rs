use tmx_utils::string_ext;
use tmx_utils::vec_string;

fn main() {
    let mut lukes_loves: Vec<String> = vec_string!["Nona", "Samantha", "Lucy", "Charles"];
    add_to_list("myself".to_string(), &mut lukes_loves);
    loop {
        match string_ext::read_string_stdin() {
            Ok(input) => add_to_list(input, &mut lukes_loves),
            Err(e) => println!("Couldn't get input, got error '{}'", e),
        };
    }
}

/// Add an item to the list and print it
fn add_to_list(input: String, list: &mut Vec<String>) {
    list.push(input);
    println!("I love {}!", string_ext::format_list(list));
}
