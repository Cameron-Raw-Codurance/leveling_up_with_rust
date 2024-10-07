use std::cmp;

use crate::attack_summary::AttackSummary;

pub struct Monster {
    health: usize,
}

impl Monster {
    pub fn take_damage(&mut self, amount: usize) -> AttackSummary {
        let damage_received = cmp::min(self.health, amount);
        self.health -= damage_received;
        AttackSummary(damage_received)
    }
}

impl Default for Monster {
    fn default() -> Self {
        Monster { health: 100 }
    }
}
