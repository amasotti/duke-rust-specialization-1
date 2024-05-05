mod models;

use crate::models::Developer;
use crate::models::Language::{Python, Rust};
use crate::models::Team::Gryffindor;

fn main() {
    let dev1 = Developer {
        first_name: "Harry".to_string(),
        last_name: "Potter".to_string(),
        age: 27,
        languages: vec![Rust, Python],
        team: Gryffindor,
    };

    println!("{:?}", dev1);
}
