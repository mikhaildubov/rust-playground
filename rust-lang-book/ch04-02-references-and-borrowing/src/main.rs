fn str_len_no_borrowing(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)
}

fn str_len_with_borrowing(s: &String) -> usize {
    s.len()
}  // s goes out of scope here but the value it points to will not be dropped as s doesn't own it

fn modify_str_no_reference(mut s: String) {
    s.push_str(", world");
    println!("{}", s);
}

//fn modify_str_with_reference_no_mut(s: &String) {
//    // This will not compile - references are immutable by default
//    s.push_str(", world");
//    println!("{}", s);
//}

fn modify_str_with_reference_and_mut(s: &mut String) {
    s.push_str(", world");
    println!("{}", s);
}

fn test_basic_borrowing() {
    // No borrowing - have to return the string back to continue using it.
    let s = String::from("hello");
    let (s, len) = str_len_no_borrowing(s);
    println!("The length of '{}' is {}.", s, len);
    
    // With borrowing - no need to return the string back.
    // &s is a reference that points to the value of s but does not own it.
    let len = str_len_with_borrowing(&s);
    println!("The length of '{}' is {}.", s, len);
}

fn test_modifying_borrowed_value() {
    let s = String::from("hello");
    modify_str_no_reference(s);

    //let s = String::from("hello");
    //modify_str_with_reference_no_mut(&s);
    
    let mut s = String::from("hello");
    modify_str_with_reference_and_mut(&mut s);
}

fn test_multiple_references() {

    // TL;DR: At any given time, you can have either one mutable reference or any number of
    // immutable references. References must always be valid.

    // Two immutable references to the same data is fine
    let s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("{}, {}", r1, r2);

    // Can't only have one mutable reference to some piece of data (to prevent data races).
    // It will only fail if you try to use both in a println!() though
    let mut s = String::from("hello");

    let _r1 = &mut s;
    let _r2 = &mut s;
    //println!("{}", _r1);
    //println!("{}", _r2);

    // Similarly you can't combine mutable and immutable references
    // Why? Users of the immutable one wouldn't expect the value would change!
    let mut s = String::from("hello");

    let _r1 = &s;
    let _r2 = &mut s;
    //println!("{}", _r1);
    //println!("{}", _r2);

    // Keep in mind that this only applies if both references are in the same scope
    let mut s = String::from("hello");

    { let _r1 = &mut s; }  // r1 goes out of scope here
    let r2 = &mut s;
    println!("{}", r2);

}

//fn dangle() -> &String {
//   let s = String::from("hello");
//    &s
//}  // At this point s goes out of scope and gets dropped so &s potentially could point to
//   // the memory given to someone else (danger! a "dangling reference"). But Rust is clever

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}

fn test_dangling_reference() {
    //let _security_breach = dangle();
    let _its_fine = no_dangle();
}

fn main() {
    test_basic_borrowing();
    test_modifying_borrowed_value();
    test_multiple_references();
    test_dangling_reference();
}
