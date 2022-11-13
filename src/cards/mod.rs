type Name = String;
type Dmg = usize;
type Hp = usize;

pub struct Card {
    name: Name,
    moves: Vec<Option<Move>>,
}

#[derive(Clone)]
pub struct Move {
    name: Name,
    dmg: Dmg,
}

impl Card {
    pub fn new(name: Name) -> Card {
        Card {
            name,
            moves: vec![None, None, None, None],
        }
    }
    pub fn add_move(mut self, m: Move) {
        'assign: for mut mv in self.moves.into_iter() {
            match mv {
                Some(_) => continue,
                None => mv = Some(m.clone()),
            }
        }
    }
}
impl Move {
    pub fn new(name: Name, dmg: Dmg) -> Move {
        Move { name, dmg }
    }
}
