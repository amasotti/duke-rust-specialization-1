fn main() {
    vec_ownership();
    vec_modifiable();
}


fn vec_ownership() {
    let n = vec![1, 2, 3, 4, 5];
    let slice = &n[1..=2];
    println!("My vector slice: {:?}", slice);
    
    // Retrieve a specific element from the slice
    println!("Element at index 0: {}", slice[0]);
    
    // Retrieve the length of the slice
    println!("Length of the slice: {}", slice.len());
}


fn vec_modifiable() {
    let mut n = vec![1, 2, 3, 4, 5];
    println!("My vector: {:?}", n);
    
    let slice = &mut n[..];
    slice[0] = 10;
    println!("My vector slice: {:?}", slice);
    
    // Retrieve the first value in the vec
    let first = slice.first().unwrap();
    println!("First value: {:?}", first);
    
    // Retrieve the last value in the vec
    let last = slice.last().unwrap();
    println!("Last value: {:?}", last);
}
