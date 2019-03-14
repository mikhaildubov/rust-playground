fn increment(x: i32) -> i32 {
    // note that there is no ';' at the end of the next line - this is to make
    // it an expression, not a statmenet (which wouldn't evaluate to a value)
    x + 1 
}

fn test_functions() {
    println!("increment(5) = {}", increment(5));
}

fn test_scope() {
    let x = 5;

    let y = {
        let x = 3;
        x * 2
    };

    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn main() {
    test_functions();
    test_scope();
}
