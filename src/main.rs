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

fn last_language<'a>(languages: &'a [String]) -> &'a str {
    languages.last().unwrap()
}

// TODO: must specific lifetime
// The reason must it is communicating to other engineers how these references are related.
fn longest_language<'a>(lang_a: &'a str, lang_b: &'a str) -> &'a str {
    if lang_a.len() >= lang_b.len() {
        lang_a
    } else {
        lang_b
    }
}

#[derive(Debug)]
struct Person<'a> {
    name: String,
    age: u8,
    friend: Option<&'a Person<'a>>,
}

fn main() {
    let current = "rust";

    let languages = vec![
        String::from("rust"),
        String::from("go"),
        String::from("elixir"),
    ];

    let result = next_language(&languages, &current);

    println!("next_language - result 1 {}", result);

    let result2 = last_language(&languages);

    println!("last_language - result 2 {}", result2);

    let result3 = longest_language("rust", "elixir");

    println!("longest_language - result 3 {}", result3);

    //  NOTE: https://thinknetcompany.github.io/learnrust/ownership.html#8-references-must-be-valid-lifetime
    let person;

    let friend = Person {
        name: String::from("Linus"),
        age: 30,
        friend: None,
    };

    person = Person {
        name: String::from("Dream"),
        age: 32,
        friend: Some(&friend),
    };

    println!("{:?}", person)
}
