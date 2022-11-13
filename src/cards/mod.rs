use std::collections::HashMap;

type Name = String;
type Dmg = usize;
type Hp = usize;

pub struct Card {
    name: Name,
    moves: Moves,
}

type Moves = HashMap<Name, Dmg>;

impl Card {
    pub fn new(name: Name) -> Card {
        Card {
            name,
            moves: HashMap::new(),
        }
    }
    pub fn add_move(&mut self, name: Name, dmg: Dmg) {
		
		self.moves.insert(name, dmg);
	}
}

#[cfg(test)]
pub mod tests {
    use super::*;
}
