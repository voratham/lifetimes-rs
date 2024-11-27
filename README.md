## Lifetime annotations
- Help the compiler make sure refs wont outlive the value they refer to

- <strong>Hardest part: </strong> this will seem like something the compiler should do on its own



### Example not work
```rs
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
    // let languages = vec![
    //     String::from("rust"),
    //     String::from("go"),
    //     String::from("elixir"),
    // ];

    // let result = next_language(&languages, "go");
    let result;
    {
        let languages = vec![
            String::from("rust"),
            String::from("go"),
            String::from("elixir"),
        ];

        result = next_language(&languages, "go");
    }

    println!("{}", result)
}

```


- If you have a function that takes in two or more refs and returns a ref 
- Rust will make a huge assumption
```rs
                    // ref to vector     // ref to value     // ref to value  
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
```

- Rust assumes that the return ref will point at data referred to by one of the arguments
- Rust will not analyze the body of your function to figure out whether the return ref is pointing at the first or second arg