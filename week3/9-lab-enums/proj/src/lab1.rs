#[derive(Debug)]
enum WineRegions {
    Bordeaux,
    Burgundy,
    Champagne,
    Tuscany,
    Rioja,
    NapaValley,
    Puglia,
}

struct Wine {
    name: String,
    region: WineRegions, // wine regions used as a type
}

fn supported_regions(w: &WineRegions) {
    match w {
        WineRegions::Rioja => println!("Rioja is supported!"),
        WineRegions::Puglia => println!("Puglia is supported and a great choice!"),
        _ => println!("{:?} is not supported!", w),
    }
}

impl WineRegions {
    fn get_popularity(&self) {
        match self {
            WineRegions::Bordeaux => println!("Bordeaux is a popular wine region!"),
            WineRegions::Burgundy => println!("Burgundy is a popular wine region!"),
            WineRegions::Champagne => println!("Champagne is a popular wine region!"),
            WineRegions::Tuscany => println!("Tuscany is a popular wine region!"),
            WineRegions::Rioja => println!("Rioja is a popular wine region!"),
            WineRegions::NapaValley => println!("Napa Valley is a popular wine region!"),
            WineRegions::Puglia => println!("Puglia is highly popular!"),
        }
    }
}

pub fn lab1() {
    let wine1 = Wine {
        name: String::from("Chateau Margaux"),
        region: WineRegions::Bordeaux,
    };

    let wine2 = Wine {
        name: String::from("Barolo"),
        region: WineRegions::Tuscany,
    };

    let wine3 = Wine {
        name: String::from("Primitivo"),
        region: WineRegions::Puglia,
    };

    // println!("Wine 1: {} from {:?}", wine1.name, wine1.region);
    // println!("Wine 2: {} from {:?}", wine2.name, wine2.region);
    supported_regions(&wine1.region);
    supported_regions(&WineRegions::Rioja);
    supported_regions(&wine3.region);

    // Challenge 2 - Popularity
    wine3.region.get_popularity();
}
