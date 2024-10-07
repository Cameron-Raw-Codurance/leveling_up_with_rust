use std::cmp;

type OnReceivedDamage = Box<dyn Fn(usize)>;

pub struct Monster {
    health: usize,
    received_damage: Vec<OnReceivedDamage>,
}

impl Monster {
    pub fn take_damage(&mut self, amount: usize) {
        let damage_received = cmp::min(self.health, amount);
        self.health -= damage_received;
        for callback in &mut self.received_damage {
            callback(damage_received);
        }
    }

    pub fn add_listener(&mut self, listener: OnReceivedDamage) {
        self.received_damage.push(listener);
    }
}

impl Default for Monster {
    fn default() -> Self {
        Monster {
            health: 100,
            received_damage: Vec::new(),
        }
    }
}
