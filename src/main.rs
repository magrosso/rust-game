use std::fmt;
use std::fmt::write;


//#[derive(Debug)]
enum Gem {
    Diamond,
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
    let gem1 = (Gem::Onyx, 25.00);
    let gem2 = (Gem::Jade, 30.50);
    let gem3 = (Gem::Ruby, 45.90);
    // array of two gems - all must be of same type
    let gems = [gem1, gem2, gem3];
    for gem in gems {
        println!("Gemstone {} costs {}", gem.0, gem.1);
    }
}
