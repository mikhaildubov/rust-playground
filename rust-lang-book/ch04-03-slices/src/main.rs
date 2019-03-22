fn test_basic_slices() {
    let s = String::from("hello, world!");

    let hello = &s[..5];     // end exclusive
    let world = &s[7..=11];  // end inclusive

    println!("{} / {}, {}", s, hello, world);
}

fn test_breaking_utf_slice() {
    let cats = String::from("😻😻");
    println!("{}", cats);

    // One cat is actually 4 bytes
    let one_cat = &cats[..4];
    println!("{}", one_cat);

    // This will panic at runtime
    //let half_a_cat = &cats[..2];
    //println!("{}", half_a_cat);
}

fn first_word(s: &String) -> &str {  // Notice the difference in types - a String slice is a &str
    for (i, &item) in s.as_bytes().iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn first_word_v2(s: &str) -> &str {  // Only difference is the type of s: it is more generic now
                                     // (string literals are &str but you can also transform a
                                     // String into &str via taking a complete slice of [..])
    for (i, &item) in s.as_bytes().iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn test_first_word() {
    println!("{}", first_word(&String::from("hello world!")));
    println!("{}", first_word(&String::from("hello")));

    // The following fails at compile time (!)
    //let mut s = String::from("hello world");
    //let word = first_word(&s);
    //s.clear();
    //println!("{}", word);
    
    println!("{}", first_word_v2("hello world!"));                     // String literals are &str
    println!("{}", first_word_v2(&String::from("hello world!")[..]));  // String slices are &str
}

fn test_array_slices() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    println!("{}", slice[1]);
}

fn main() {
    test_basic_slices();
    test_breaking_utf_slice();
    test_first_word();
    test_array_slices();
}
