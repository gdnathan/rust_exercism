pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        if self.health <= 0 {
            Some(Player {
                health: 100,
                mana: if self.level < 10 { None } else { Some(100) },
                level: self.level,
            })
        } else {
            None
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        if self.mana == None {
            if self.health > mana_cost {
                self.health -= mana_cost;
            } else {
                self.health = 0;
            }
            0
        } else if self.mana.unwrap() > mana_cost {
            self.mana = self.mana.map(|x| x - mana_cost);
            mana_cost * 2
        } else {
            0
        }
    }
}
