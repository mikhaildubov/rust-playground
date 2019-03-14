fn test_string_types() {
    let mut _s = "hello";      // str (immutable)
    _s = "hi";
    //_s = String::from("hi"); // String (mutable) - a different type so will fail
    //println!(_s);            // will also fail, you have to pass an actual literal
    println!("{}", _s);

    let mut s = String::from("hello");
    s.push_str(", wolrd!");
    println!("{}", s);
}

fn test_ownership_basics() {
    // Strings are stored on the heap and so the concept of ownership
    // does apply to them
    let s1 = String::from("hello");  // s1 owns the String on the heap
    let s2 = s1;                     // s2 now owns the String, s1 gets invalidated
    //println!("{}, world!", s1);    // will fail as s1 is no longer valid
    println!("{}, world!", s2);      // heap memory owned by s2 gets freed at this point

    // The same thing works just fine with integers, as they are stored on the stack
    // (not on the heap) and so there is no need to free up any heap memory when it goes
    // out of scope / care about ownership of any memory in the heap (= they have the `Copy` trait)
    let i1 = 5;
    let i2 = i1;
    println!("{}", i1);
    println!("{}", i2);
}

fn test_clone() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("{}, world!", s1);
    println!("{}, world!", s2);
}

fn test_taking_ownership() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    //println!("{}", s);            // (so this call would fail)

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it’s okay to still use x afterwards

    println!("{}", x);              // ... like that

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn test_giving_ownership() {
    let s1 = gives_ownership();         // gives_ownership moves its return value into s1

    println!("{}", s1);

    let mut s2 = String::from("hello"); // s2 comes into scope

    s2 = takes_and_gives_back(s2);      // s2 is moved into takes_and_gives_back,
                                        // which also moves its return value back into s2
    println!("{}", s2);
} // Here, s1 and s2 go out of scope and are dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function that calls it

    let some_string = String::from("hello"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling function
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}

fn main() {
    test_string_types();
    test_ownership_basics();
    test_clone();
    test_taking_ownership();
    test_giving_ownership();
}
