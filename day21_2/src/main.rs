use std::fs;

#[derive(Clone, Debug)]
struct Item {
    cost: i32,
    damage: i32,
    armor: i32,
}

#[derive(Clone, Debug)]
struct Unit {
    hp: i32,
    weapon: Item,
    armor: Item,
    rings: Vec<Item>,
}

trait BattleUnit {
    fn stats(&self) -> (i32, i32);
}

impl BattleUnit for Unit {
    fn stats(&self) -> (i32, i32) {
        let mut s = (0, 0);
        s.0 += &self.weapon.damage;

        s.1 += &self.armor.armor;

        for ring in &self.rings {
            s.0 += ring.damage;
            s.1 += ring.armor;
        }

        return s;
    }
}

fn main() {
    let weapons = [
        Item {
            cost: 0,
            damage: 0,
            armor: 0,
        },
        Item {
            cost: 8,
            damage: 4,
            armor: 0,
        },
        Item {
            cost: 10,
            damage: 5,
            armor: 0,
        },
        Item {
            cost: 25,
            damage: 6,
            armor: 0,
        },
        Item {
            cost: 40,
            damage: 7,
            armor: 0,
        },
        Item {
            cost: 74,
            damage: 8,
            armor: 0,
        },
    ];

    let armors = [
        Item {
            cost: 0,
            damage: 0,
            armor: 0,
        },
        Item {
            cost: 13,
            damage: 0,
            armor: 1,
        },
        Item {
            cost: 31,
            damage: 0,
            armor: 2,
        },
        Item {
            cost: 53,
            damage: 0,
            armor: 3,
        },
        Item {
            cost: 75,
            damage: 0,
            armor: 4,
        },
        Item {
            cost: 102,
            damage: 0,
            armor: 5,
        },
    ];

    let rings = [
        Item {
            cost: 0,
            damage: 0,
            armor: 0,
        },
        Item {
            cost: 0,
            damage: 0,
            armor: 0,
        },
        Item {
            cost: 25,
            damage: 1,
            armor: 0,
        },
        Item {
            cost: 50,
            damage: 2,
            armor: 0,
        },
        Item {
            cost: 100,
            damage: 3,
            armor: 0,
        },
        Item {
            cost: 20,
            damage: 0,
            armor: 1,
        },
        Item {
            cost: 40,
            damage: 0,
            armor: 2,
        },
        Item {
            cost: 80,
            damage: 0,
            armor: 3,
        },
    ];

    let mut enemy = Unit {
        hp: 0,
        weapon: weapons[0].clone(),
        armor: armors[0].clone(),
        rings: vec![],
    };

    for line in fs::read_to_string("../inputs/day21.txt")
        .unwrap()
        .trim()
        .lines()
    {
        let parts = line.split(": ").collect::<Vec<_>>();
        match parts[0] {
            "Hit Points" => enemy.hp = parts[1].parse().unwrap(),
            "Damage" => enemy.rings.push(Item {
                cost: 0,
                damage: parts[1].parse().unwrap(),
                armor: 0,
            }),
            "Armor" => enemy.rings.push(Item {
                cost: 0,
                damage: 0,
                armor: parts[1].parse().unwrap(),
            }),
            _ => (),
        }
    }

    let mut result = 0;
    for weapon in &weapons[1..] {
        for armor in &armors {
            for i in 0..rings.len() - 1 {
                for j in i + 1..rings.len() {
                    let cost = weapon.cost + armor.cost + rings[i].cost + rings[j].cost;
                    if cost <= result {
                        continue;
                    }

                    let player = Unit {
                        hp: 100,
                        weapon: weapon.clone(),
                        armor: armor.clone(),
                        rings: vec![rings[i].clone(), rings[j].clone()],
                    };

                    if !check(player.clone(), enemy.clone()) {
                        result = cost;
                    }
                }
            }
        }
    }

    println!("{}", result);
}

fn check(mut player: Unit, mut enemy: Unit) -> bool {
    loop {
        enemy.hp -= 1.max(player.stats().0 - enemy.stats().1);
        if enemy.hp <= 0 {
            return true;
        }

        player.hp -= 1.max(enemy.stats().0 - player.stats().1);
        if player.hp <= 0 {
            return false;
        }
    }
}
