mod size_formatter;

#[derive(Debug)]
enum Language {
    Rust,
    Python,
    Kotlin,
}

enum Color {
    Red(u8, u8, u8),
    Green(u8, u8, u8),
    Blue(u8, u8, u8),
}

#[derive(Debug)]
struct Developer {
    name: String,
    lang: Language,
}

impl Developer {
    fn new(name: &str, lang: Language) -> Developer {
        Developer {
            name: name.to_string(),
            lang,
        }
    }

    fn assign_lang(&mut self, lang: Language) {
        self.lang = lang;
    }

    fn get_lang(&self) -> &Language {
        &self.lang
    }
}

fn explore_eunms() {
    let lang = Language::Rust;

    match lang {
        Language::Rust => println!("You use Rust, good choice!"),
        Language::Python => println!("Python, easy but slow!"),
        Language::Kotlin => println!("Kotlin, Java's little and better brother!"),
    }

    // Enum with values
    let color = Color::Red(255, 0, 0);
    match color {
        Color::Red(r, g, b) => println!("Red color: R: {}, G: {}, B: {}", r, g, b),
        Color::Green(r, g, b) => println!("Green color: R: {}, G: {}, B: {}", r, g, b),
        Color::Blue(r, g, b) => println!("Blue color: R: {}, G: {}, B: {}", r, g, b),
    }

    let mut dev = Developer::new("John", Language::Rust);
    println!("Developer: {:?}", dev);

    dev.assign_lang(Language::Python);
    println!("Developer: {:?}", dev);
}

fn main() {
    explore_eunms();

    // Nice application of enums
    size_formatter::run();
}

