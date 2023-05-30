pub static STRENGTH: usize = 0;
pub static DEXTERITY: usize = 1;
pub static CONSTITUTION: usize = 2;
pub static INTELLIGENCE: usize = 3;
pub static WISDOM: usize = 4;
pub static CHARISMA: usize = 5;

pub struct Attributes {
    stats: [i32; 6],
}

impl Default for Attributes {
    fn default() -> Self {
        Self {
            stats: [10, 10, 10, 10, 10, 10],
        }
    }
}

impl Attributes {
    #[must_use]
    pub fn get_attr(&self, attr: usize) -> i32 {
        self.stats[attr]
    }

    #[must_use]
    pub fn get_attr_mod(&self, attr: usize) -> i32 {
        (self.stats[attr] / 2) - 5
    }
}
