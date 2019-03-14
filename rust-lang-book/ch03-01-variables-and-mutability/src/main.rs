const TEN_THOUSAND: u32 = 10_000;

fn test_raw_identifiers() {
    let r#fn = "test";
    //let fn = "test";
    println!("{}", r#fn);
}

fn test_immutable_variables() {
    //let x = 5;
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}

fn test_constants() {
    println!("The value of TEN_THOUSAND is: {}", TEN_THOUSAND);
}

fn test_shadowing() {
    let x = 5;
    //x = x + 1;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);

    let spaces = "   ";         // string
    let spaces = spaces.len();  // integer
    println!("The value of spaces is: {}", spaces);
}

fn main() {
    test_raw_identifiers();
    test_immutable_variables();
    test_constants();
    test_shadowing();
}
