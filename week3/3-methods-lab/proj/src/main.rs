mod model;

use crate::model::User;

fn main() {
    let mut new_user = User::new(
        String::from("alfredodeza"),
        String::from("alfreodeza@example.com"),
        String::from("https://alfredodeza.com"),
    );
    println!("Hello, {}!", new_user.username);
    println!(
        "Account {} status is: {}",
        new_user.username, new_user.active
    );
    new_user.deactivate();
    println!(
        "Account {} status is: {}",
        new_user.username, new_user.active
    );

    let mut toni = User::from_email("toni@rust-test.com");
    println!("User: {:?}", toni);
    println!("Email: {:?}", toni.email());

    toni.update_uri("https://toni.com");
    println!("User: {:?}", toni);
}
