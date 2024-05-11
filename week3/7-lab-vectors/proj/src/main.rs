use rand::Rng;

fn get_item(index: usize) {
    //let index = 3; // this looks like an unsigned integer, but it's actually a usize
    let vec = vec![1, 2, 3, 4, 5];

    // Retrieve a value at a specific index
    let value = vec.get(index).unwrap();

    // print the value
    println!("The value at index {} is {:?}", index, value);
}

fn challenge_1_empty_vector(vec: Vec<i32>, index: usize) {

    // Check if the vector is empty
    if vec.len() == 0 {
        println!("The vector is empty!");
        return;
    }

    // Retrieve a value at a specific index
    match vec.get(index) {
        Some(value) => println!("The value at index {} is: {}", index, value),
        None => println!("Element {} does not exist in the vector", index),
    }
}

fn sum_of_items(vec: Vec<i32>) {
    // Calculate the sum of all items in the vector
    let sum: i32 = vec.iter().sum();
    println!("The sum of all items in the vector is: {}", sum);
}

// Extra learning: generate random vec with `rand`
fn generate_random_vec(size:usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let random_vec: Vec<i32> = (0..size).map(|_| rng.gen_range(1..=20)).collect();
    println!("Random vector: {:?}", random_vec);
    random_vec
}

fn lab_1_play_with_vecs() {
    let vec = vec![1, 2, 3, 4, 5];
    get_item(3);

    // Retrieve a value at a specific index
    let third_value = vec[2];
    println!("The third value in the vector is: {}", third_value);

    // Retrieve the last value
    let last_value = vec.last().unwrap();
    println!("The last value in the vector is: {}", last_value);

    // Retrieve the first value using pattern matching
    match vec.first() {
        Some(first_value) => println!("The first value in the vector is: {}", first_value),
        None => println!("The vector is empty!"),
    }

    // Challenge 1
    challenge_1_empty_vector(vec![], 3);

    // Challenge 2
    let v = generate_random_vec(7);
    sum_of_items(v);
}

/// Lab 2 on vectors 
fn lab_2_add_elements_to_vec() {
    let mut v = vec![1, 2, 3];
    println!("{:?}", v); // Output: [1, 2, 3]
    
    v.push(4);
    println!("{:?}", v); // Output: [1, 2, 3, 4]

    // extend adds each element of the given slice to the vector
    let more_numbers = vec![5, 6];
    v.extend(more_numbers);
    println!("{:?}", v);

    // append adds the given vector to the vector, requires the vector to be mutable
    let mut other_numbers = vec![7, 8];
    v.append(&mut other_numbers);
    println!("{:?}", v);

    // insert items at a given index
    v.insert(0, 150);
    println!("{:?}", v); // Output: [0, 1, 2, 3, 4, 5, 6, 7, 8]
    
    // Challenge 1: Insert at the beginning
    pad_vector(&mut v, 100);
    println!("{:?}", v); // Output: [100, 0, 1, 2, 3, 4, 5, 6, 7, 8, 100]
    
    // Challenge 2: Append a vector
    let v2 = vec![200, 300];
    merge_vectors(&mut v, v2);
    println!("{:?}", v); // Output: [100, 0, 1, 2, 3, 4, 5, 6, 7, 8, 100, 200, 300]
}

fn pad_vector(vector: &mut Vec<i32>, value: i32) {
    vector.insert(0, value);
    vector.insert(vector.len(), value);
}

fn merge_vectors(v1: &mut Vec<i32>, v2: Vec<i32>) {
    v1.extend(v2);
}

fn main() {
    lab_2_add_elements_to_vec();
}