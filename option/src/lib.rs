// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn new(level: u32) -> Self {
        Self {
            health: 100,
            mana: if level < 10 {None} else {Some(100)},
            level: level
        }
    }

    pub fn revive(&self) -> Option<Player> {
        match self.health {
            0 => Some(Player::new(self.level)),
            _ => None
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            Some(x) => {
                if x >= mana_cost {
                    self.mana = Some(x - mana_cost);
                    return 2 * mana_cost;
                } else {0}
            }
            None => {
                self.health = self.health - std::cmp::min(self.health, mana_cost);
                return 0;
            }
        }
    }
}
