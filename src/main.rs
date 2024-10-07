use attack_summary::AttackSummary;
use damage_counter::DamageCounter;
use monster::Monster;
use rand::Rng;

mod attack_summary;
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
        let AttackSummary(damage_received) = target.take_damage(damage);
        counter.on_damage_received(damage_received);

        println!("Monster {} received {} damage", index, damage);
    }
}
