// TODO: solution
fn next_language<'a>(languages: &'a [String], current: &str) -> &'a str {
    let mut found = false;

    for lang in languages {
        if found {
            return lang;
        }

        if lang == current {
            found = true;
        }
    }

    languages.last().unwrap()
}

fn main() {
    let current = "go";

    let languages = vec![
        String::from("rust"),
        String::from("go"),
        String::from("elixir"),
    ];

    let result = next_language(&languages, &current);

    println!("{}", result)
}
