/// Basic loop example
pub fn loop_while() {
    let mut counter = 0;
    while counter < 10 {
        println!("Counter: {}", counter);
        counter += 1;
    }
}

pub fn loop_for() {
    for number in 0..10 {
        println!("Number: {}", number);
    }
}

pub fn loop_for_each() {
    let numbers = [1, 2, 3, 4, 5];
    for number in numbers.iter() {
        println!("Number: {}", number);
    }
}

pub fn loop_for_each_mut() {
    let mut numbers = [1, 2, 3, 4, 5];
    for number in numbers.iter_mut() {
        *number += 1;
        println!("Number: {}", number);
    }
}

pub fn loop_for_each_rev() {
    let numbers = [1, 2, 3, 4, 5];
    for number in numbers.iter().rev() {
        println!("Number: {}", number);
    }
}

pub fn loop_for_each_rev_mut() {
    let mut numbers = [1, 2, 3, 4, 5];
    for number in numbers.iter_mut().rev() {
        *number += 1;
        println!("Number: {}", number);
    }
}

pub fn loop_loop() {
    for i in 0..3 {
        for j in 0..3 {
            println!("i: {}, j: {}", i, j);
        }
    }
}

pub fn loop_while_break() {
    let mut counter = 0;
    while counter < 10 {
        println!("Counter: {}", counter);
        counter += 1;
        if counter == 5 {
            break;
        }
    }
}

pub fn named_loop() {
    'outer: for i in 0..3 {
        'inner: for j in 0..3 {
            println!("i: {}, j: {}", i, j);
            if i == 1 && j == 1 {
                break 'outer;
            }
        }
    }
}