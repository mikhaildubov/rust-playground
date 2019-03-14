fn test_integers() {
    // a-e are all the same integer, defined using different literals
    let a:u16 = 10000;          // would be i32 by default
    let b = 10_000u16;          // can use '_' and type definitions
    let c = 0x2710;             // hexadecimal
    let d = 0o23420;            // octal
    let e = 0b0010011100010000; // binary
    let f = b'A';               // byte (u8 only)
    //let g:u8 = 256;           // will not compile (overflow)
    //let h:u16 = 128u8;        // will not compile (types mismatch)
    println!("{} {} {} {} {} {}", a, b, c, d, e, f);
}

fn test_floats() {
    let a = 3.0;             // f64
    let b:f32 = 3.0;         // f32
    let c:f64 = b.into();    // a lossless cast from f32 to f64
    let d:f32 = 3i16.into(); // a lossless cast from i16 to f32. Fails for i32
    let e:f64 = 3i32.into(); // a lossless cast from i32 to f64
    //let f:f32 = c.into();  // will not compile (case from f64 to f32)
    //let g:f64 = b;         // will not compile (types mismatch)
    //let h:f64 = 3;         // will not compile (types mismatch)
    println!("{} {} {} {} {}", a, b, c, d, e);
}

fn test_operations() {
    let sum = 5 + 10;       // integer sum
    let diff = 95.5 - 1.0;  // float diff
    //let diff = 95.5 - 1;  // fails (float - integer)
    let prod = 30.5 * 2.0;  // float product
    //let prod = 30.5 * 2;  // fails (float * integer)
    let div1 = 34.3 / 2.0;  // float division
    //let div1 = 34.3 / 2;  // fails (float / integer)
    let div2 = 35 / 2;      // integer division - returns 17
    let rem1 = 34.3 % 2.0;  // returns 0.3 (well, almost)
    //let rem1 = 34.3 % 2;  // fails (float % integer)
    let rem2 = 35 % 2;      // integer mod - returns 1
    println!("{} {} {} {} {} {} {}", sum, diff, prod, div1, div2, rem1, rem2);
}

fn test_booleans() {
    let t = true;
    let f: bool = !t;
    //let f2: bool = 0.into();  // doesn't work for i32
    println!("{} {}", t, f);
}

fn test_characters() {
    // Rust's char is a Unicode Scalar Value
    let z1 = 'Z';
    let z2 = 'ℤ';
    let cat = '😻';
    println!("{} {} {}", z1, z2, cat);
}

fn test_compounds() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    //println!("{}", tup);    // fails
    let (x, y, z) = tup;      // pattern matching
    println!("({}, {}, {})", x, y, z);
    println!("({}, {}, {})", tup.0, tup.1, tup.2);

    // arrays have fixed length (unlike vectors), but are allocated as a single
    // chunk of memory on the stack (and not in the heap).
    let arr: [i32; 3] = [1, 2, 3];
    //println!("{}", arr);    // fails
    println!("[{}, {}, {}]", arr[0], arr[1], arr[2]);
    //println!("{}", arr[3]); // compile-time error
    let i = 3;
    println!("{}", arr[i]);   // run-time error
}

fn main() {
    test_integers();
    test_floats();
    test_operations();
    test_booleans();
    test_characters();
    test_compounds();
}
