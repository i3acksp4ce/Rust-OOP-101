trait Weapon {
    fn attack(&self);
}

struct Sword;
struct Staff;
struct Dagger;

impl Weapon for Sword {
    fn attack(&self) {
        println!("Swinging sword");
    }
}

impl Weapon for Staff {
    fn attack(&self) {
        println!("Casting spell");
    }
}

impl Weapon for Dagger {
    fn attack(&self) {
        println!("Stabbing with dagger");
    }
}

struct HeathStatus {
    health: u8,
}

trait HeathUtils {
    fn increase_health(&mut self, amount: u8);
    fn decrease_health(&mut self, amount: u8);
    fn get_health(&self) -> u8;
    // fn set_health(&mut self, health: u8);
}

impl HeathUtils for HeathStatus {
    fn increase_health(&mut self, amount: u8) {
        let new_health = self.get_health().saturating_add(amount).min(100);
        self.health = new_health;
    }

    fn decrease_health(&mut self, amount: u8) {
        let new_health = self.get_health().saturating_sub(amount);
        self.health = new_health;
    }

    fn get_health(&self) -> u8 {
        self.health
    }
}

struct Warrior {
    health: Box<dyn HeathUtils>,
    weapon: Box<dyn Weapon>,
}

impl Warrior {
    fn new() -> Self {
        Self {
            health: Box::new(HeathStatus { health: 100 }),
            weapon: Box::new(Sword),
        }
    }
}

struct Mage {
    health: Box<dyn HeathUtils>,
    weapon: Box<dyn Weapon>,
}

impl Mage {
    fn new() -> Self {
        Self {
            health: Box::new(HeathStatus { health: 100 }),
            weapon: Box::new(Staff),
        }
    }
}

struct Rouge {
    health: Box<dyn HeathUtils>,
    weapon: Box<dyn Weapon>,
}

impl Rouge {
    fn new() -> Self {
        Self {
            health: Box::new(HeathStatus { health: 100 }),
            weapon: Box::new(Dagger),
        }
    }
}

fn special_attack(weapon: Box<dyn Weapon>) {
    weapon.attack();
}

fn main() {
    let mut warrior = Warrior::new();
    let mut mage = Mage::new();
    let mut rouge = Rouge::new();

    warrior.health.decrease_health(10);
    mage.health.decrease_health(10);
    rouge.health.decrease_health(10);

    println!("Warrior health: {}", warrior.health.get_health());
    println!("Mage health: {}", mage.health.get_health());
    println!("Rouge health: {}", rouge.health.get_health());

    warrior.health.increase_health(10);
    mage.health.increase_health(10);
    rouge.health.increase_health(10);

    println!("Warrior health: {}", warrior.health.get_health());
    println!("Mage health: {}", mage.health.get_health());
    println!("Rouge health: {}", rouge.health.get_health());

    special_attack(warrior.weapon);
    special_attack(mage.weapon);
    special_attack(rouge.weapon);
}
