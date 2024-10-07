#[derive(Default)]
pub struct DamageCounter {
    damage_inflicted: usize,
}

impl DamageCounter {
    pub fn reached_target_damage(&self) -> bool {
        self.damage_inflicted > 100
    }

    pub fn on_damage_received(&mut self, damage: usize) {
        self.damage_inflicted += damage;
    }
}
