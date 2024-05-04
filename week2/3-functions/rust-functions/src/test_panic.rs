
pub fn demo_panic() {
    let n = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    loop_over_array(&n);
    
    // let empty_array = [];
    // loop_over_array(&empty_array);
}

fn loop_over_array(v: &[i32]) {
    if v.len() == 0 {
        panic!("The array is empty.. Check the input!");
    }
    
    for i in 0..v.len() {
        println!("Element at index {}: {}", i, v[i]);
    }
}