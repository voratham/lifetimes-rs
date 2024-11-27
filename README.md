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



## Super common question
- Why does it matter whether the return ref points at the first or second arg?
- Why doesn't rust analyze the function body to figure out if the returned ref points at the first or second arg?


* You're using a library that implements a `split` function 
* The function signature makes it clear that the returned ref will be tied to the first arg.

```rs
fn split<'a>(s: &'a str, pattern: &str) -> &'a str
```

-  The example work in case lifetime when specific

```rs
fn main(){
    let sentence = "hi how are you"; // HERe
    let result;
    {
        let pattern = " ";s
        result = split(sentence , pattern)
    }

    println!("{}", resukt)
}
```

- The example not work

```rs
fn main(){
    let pattern = " "; // HERE not work because pattern is second argument and not specific lifetime
    let result;
    {
        let sentence = "hi how are you ";
        result = split(sentence , pattern)
    }

    println!("{}", result)
}
```

- if we relied on the Rust to figure out the lifetimes, we wouldn't know if the returned ref uses the first or second arg




## Omitting lifetime annotations is referred to aas elision (elision word you can see on rust document term !!!)
i.e. i removed the lifetime annotations -> i elided the lifetime annotations



https://thinknetcompany.github.io/learnrust/ownership.html
```rs
// TODO: Fix the compiler errors without changing anything except adding or
// removing references (the character `&`).

// Shouldn't take ownership
fn get_char(data: String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: &String) {
    data = data.to_uppercase();

    println!("{data}");
}

fn main() {
    let data = "Rust is great!".to_string();

    get_char(data);

    string_uppercase(&data);
}
```
