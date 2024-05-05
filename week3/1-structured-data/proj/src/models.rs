#[derive(Debug)]
pub struct Developer {
    pub first_name: String,
    pub last_name: String,
    pub age: u8,
    pub languages: Vec<Language>,
    pub team: Team,
}

#[derive(Debug)]
#[allow(unused)]
pub enum Language {
    Rust,
    Python,
    C,
    Cpp,
    Go,
    Ruby,
    Swift,
    Kotlin,
    TypeScript,
    Php,
    Other(String),
}

#[derive(Debug)]
#[allow(unused)]
pub enum Team {
    Slytherin,
    Hufflepuff,
    Gryffindor,
    Ravenclaw,
}
