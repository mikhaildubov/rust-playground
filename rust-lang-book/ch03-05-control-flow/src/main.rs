fn test_if_branches() {
    let x = 5;
    if x > 0 {
        println!("x is positive");
    } else if x == 0 {
        println!("x is zero");
    } else {
        println!("x is negative");
    }
}

fn test_if_evaluation() {
    let x = 5;
    let y = if x > 0 { 1 } else { -1 };
    //let y = if x > 0 { 1 } else { "-1" };  // fails (type mismatch)
    println!("The value of y is: {}", y);
}

fn test_loop_loop() {
    let mut cnt = 2;
    let res = loop {
        cnt *= 2;
        if cnt > 2000 {
            break cnt;  // the break expression can evaluate the loop to a value
        }
    };
    println!("The loop returned {}", res);
    assert_eq!(res, 2048);
}

fn test_while_loop() {
    let mut cnt = 2;
    while cnt < 2000 {
        cnt *= 2;
    };
    println!("cnt is {}", cnt);
    assert_eq!(cnt, 2048);
}

fn test_for_loop() {
    println!("Looping through an array...");
    let arr = [1, 2, 3];
    for elem in arr.iter() {
        println!("the value is: {}", elem);
    }
    for elem in arr.iter().rev() {
        println!("the value is: {}", elem);
    }
    for elem in &arr {
        println!("the value is: {}", elem);
    }
    // the following fails
    //for elem in arr {
    //    println!("the value is: {}", elem);
    //}
    
    println!("Looping through a range...");
    for elem in 1..4 {
        println!("the value is: {}", elem);
    }
    for elem in 4..1 {  // reverse ranges do not work - `cargo clippy` will catch it
        println!("the value of {} will never be printed", elem);
    }
    for elem in (1..4).rev() {
        println!("the value is: {}", elem);
    }
}

fn main() {
    test_if_branches();
    test_if_evaluation();
    test_loop_loop();
    test_while_loop();
    test_for_loop();
}
