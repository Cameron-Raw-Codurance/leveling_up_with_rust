use std::cmp;

pub struct Monster {
    health: usize,
}

impl Monster {
    pub fn take_damage(&mut self, amount: usize, on_damage_received: impl FnOnce(usize)) {
        let damage_received = cmp::min(self.health, amount);
        self.health -= damage_received;
        on_damage_received(damage_received);
    }
}

impl Default for Monster {
    fn default() -> Self {
        Monster { health: 100 }
    }
}
