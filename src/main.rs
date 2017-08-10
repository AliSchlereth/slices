fn main() {
    let string = String::from("the quick brown fox");
    let first = first_word(&string);
    let second = second_word(&string);
    println!("{:?}", string);
    println!("{:?}", first);
    println!("{:?}", second);
}

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();
//
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }
//
//     s.len()
// }

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn second_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    let mut space_count = 0;
    let mut beginning = 0;

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' && space_count == 0 {
            beginning = i + 1;
            space_count = 1;
        } else if item == b' ' {
            return &s[beginning..i]
        }
    }
    &s[..]
}
