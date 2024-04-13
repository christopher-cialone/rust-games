use std::fmt;

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use std::io;

#[derive(Debug, FromPrimitive)]
enum Gem {
    Diamond = 1,
    Sapphire,
    Ruby,
    Topaz,
    Onyx,
    Jade,
}

impl fmt::Display for Gem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Gem::Diamond => write!(f, "Diamond"),
            Gem::Sapphire => write!(f, "Sapphire"),
            Gem::Ruby => write!(f, "Ruby"),
            Gem::Topaz => write!(f, "Topaz"),
            Gem::Onyx => write!(f, "Onyx"),
            Gem::Jade => write!(f, "Jade"),
        }
    }
}

fn main() {
    let mut map = [[0; 5]; 5];
    println!("{map:?}");

    map[4][2] = 1;
    map[1][2] = 2;
    map[3][3] = 3;
    map[0][2] = 4;
    map[1][4] = 5;

    for row in map {
        println!("{row:?}");
    }

    let mut found: Vec<Gem> = Vec::new();

    println!("Search an X Y position (Example inoput: 4 3):");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let parts: Vec<&str> = input.trim().split_ascii_whitespace().collect();

    if parts.len() != 2 {
        println!("Two numbers required");
    }

    let (x, y) = match (parts[0].parse::<u8>(), parts[1].parse::<u8>()) {
        (Ok(x), Ok(y)) => (x, y),
        _ => {
            println!("Something went wrong");
            return;
        }

    };

    let data = map[x as usize][y as usize];
    found.push(Gem::from_u8(data).expect("No gem found"));
    println!("{data:?}");
}
