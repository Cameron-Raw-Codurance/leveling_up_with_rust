use damage_counter::DamageCounter;
use monster::Monster;
use rand::Rng;

mod damage_counter;
mod monster;

fn main() {
    let mut rng = rand::thread_rng();
    let mut counter = DamageCounter::default();
    let mut monsters: Vec<_> = (0..5).map(|_| Monster::default()).collect();

    while !counter.reached_target_damage() {
        let index = rng.gen_range(0..monsters.len());
        let target = &mut monsters[index];

        let damage = rng.gen_range(0..50);
        target.take_damage(damage, |dmg| counter.on_damage_received(dmg));

        println!("Monster {} received {} damage", index, damage);
    }
}
