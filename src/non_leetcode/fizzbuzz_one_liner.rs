pub fn fizzbuzz(size: i32) -> Vec<String> {
    (0..=size)
        .map(|x| match x {
            x if x % 3 == 0 && x % 5 == 0 => String::from("fizzbuzz"),
            x if x % 5 == 0 => String::from("buzz"),
            x if x % 3 == 0 => String::from("fizz"),
            x => x.to_string(),
        })
        .collect()
}

/*
ONE-LINE FORM
fn fizzbuzz(size: i32) -> Vec<String> { (0..size).map(|x| match x { x if x % 3 == 0 && x % 5 == 0 => String::from("fizzbuzz"), x if x % 5 == 0 => String::from("buzz"), x if x % 3 == 0 => String::from("fizz"), x => x.to_string(), }).collect() }
 */
