#[derive(Debug)]
#[allow(dead_code)]
struct Person {
    first_name: String,
    last_name: String,
    age: u8,
    email: Option<String>,
    telephone: Option<String>,
}

impl Person {
    fn get_full_name(&self) -> String {
        self.first_name.clone() + " " + &self.last_name.clone()
        // Alternative and more idiomatic would be:
        // format!("{} {}", self.first_name, self.last_name)
    }
}

/// Same as above but pretending not to know about impl
fn get_person_full_name(person: &Person) -> String {
    person.first_name.clone() + " " + &person.last_name.clone()
}


fn main() {
    let person = Person {
        first_name: "Toni".to_string(),
        last_name: "Masotti".to_string(),
        age: 25,
        email: Some("cool@rust.org".to_string()),
        telephone: None,
    };

    // Task 1: Print the person struct after adding email and telephone fields
    println!("{:?}", person);

    // Task 2: Create an impl for the Person struct
    println!("Full name (impl): {}", person.get_full_name());

    // Task 3: Create a function that takes a Person struct and returns the full name
    println!("Full name (fn): {}", get_person_full_name(&person));
}
