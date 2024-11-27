fn next_language(languages: &[String], current: &str) -> &str {
    let mut found = false;

    for lang in languages {
        if found {
            return lang;
        }

        if lang == current {
            found = true;
        }
    }

    languages.last().unwrap();
}

fn main() {
    let languages = vec![
        String::from("rust"),
        String::from("go"),
        String::from("elixir"),
    ];

    let result = next_language(&languages, "go");

    println!("{}", result)
}
