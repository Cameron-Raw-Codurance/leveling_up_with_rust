use std::{cell::RefCell, rc::Rc};

use damage_counter::DamageCounter;
use monster::Monster;
use rand::Rng;

mod damage_counter;
mod monster;

fn main() {
    let mut rng = rand::thread_rng();
    let counter = Rc::new(RefCell::new(DamageCounter::default()));
    let mut monsters: Vec<_> = (0..5).map(|_| Monster::default()).collect();

    for monster in &mut monsters {
        let counter_rc_clone = Rc::clone(&counter);
        monster.add_listener(Box::new(move |damage| {
            counter_rc_clone.borrow_mut().on_damage_received(damage);
        }));
    }

    while !counter.borrow().reached_target_damage() {
        let index = rng.gen_range(0..monsters.len());
        let target = &mut monsters[index];

        let damage = rng.gen_range(0..50);
        target.take_damage(damage);

        println!("Monster {} received {} damage", index, damage);
    }
}
