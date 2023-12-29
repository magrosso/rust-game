use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::{fmt, io};
use std::collections::HashMap;


#[derive(Debug, FromPrimitive, Eq, PartialEq, Hash)]
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

fn play_game(map: &mut [[u8; 5]; 5]) -> Vec<Gem> {
    let row_count = map.len();
    let col_count = map[0].len();
    let field_count = row_count * col_count;

    let mut hidden_gem_count = 0;
    for row in *map {
        println!("{row:?}");
        for col in row {
            if col != 0 {
                hidden_gem_count += 1;
            }
        }
    }
    println!("{hidden_gem_count} gems hidden in {field_count} fields");

    println!("Input search position by X,Y coordinates, e.g.: 3 4:");
    let mut found_gems: Vec<Gem> = Vec::new();
    let mut attempt_count = 0;

    // loop until all gems found
    while found_gems.len() < map.len() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Input error!");
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        if parts.len() != 2 {
            println!("Error parsing number! - Enter two space separated numbers: 0..{row_count} 0..{col_count}");
            continue;
        }
        let (x, y) = match (parts[0].parse::<u8>(), parts[1].parse::<u8>()) {
            (Ok(x), Ok(y)) => (x, y),
            _ => {
                println!("Error parsing number! - Enter two space separated numbers: 0..{row_count} 0..{col_count}");
                continue;
            }
        };
        if x as usize >= row_count || y as usize >= col_count {
            println!("Error parsing number! - Enter two space separated numbers: 0..{row_count} 0..{col_count}");
            continue;
        }
        attempt_count += 1;
        let data_at_pos: u8 = map[x as usize][y as usize];
        let gem: Gem = match Gem::from_u8(data_at_pos) {
            Some(gem) => gem,
            None => {
                println!("{attempt_count}. No gem found at position {x} {y} - Try again!");
                continue;
            }
        };
        found_gems.push(gem);
        map[x as usize][y as usize] = 0;
        println!("{attempt_count}. Gem found at position {x} {y}. In your bag are: {found_gems:?} - {} remaining gems to find!", hidden_gem_count - found_gems.len())
    }
    found_gems
}

fn main() {
    let mut map: [[u8; 5]; 5] = [[0; 5]; 5];
    map[0][4] = 1;
    map[1][3] = 2;
    map[3][2] = 4;
    map[4][1] = 3;
    map[2][2] = 6;

    let found_gems = play_game(&mut map);

    println!("Congratulations - You found all hidden gems!");
    let mut gem_values: HashMap<Gem, f64> = HashMap::new();
    gem_values.insert(Gem::Diamond, 1000.0);
    gem_values.insert(Gem::Onyx, 100.0);
    gem_values.insert(Gem::Sapphire, 6000.0);
    gem_values.insert(Gem::Ruby, 300.0);
    gem_values.insert(Gem::Jade, 400.0);
    gem_values.insert(Gem::Topaz, 50.0);
    let mut gem_value_sum = 0.0;
    for gem in found_gems {
        gem_value_sum += gem_values[&gem];
    }
    println!("Total value of your gem bag: {gem_value_sum} €")
}
