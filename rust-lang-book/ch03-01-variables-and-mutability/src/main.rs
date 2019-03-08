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

fn main() {
    test_raw_identifiers();
    test_immutable_variables();
}
