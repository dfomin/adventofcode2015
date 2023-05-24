use std::fs;

#[derive(Clone, Debug)]
struct Boss {
    hp: i32,
    damage: i32,
    poison: i32,
}

#[derive(Clone, Debug)]
struct Player {
    hp: i32,
    mana: i32,
    shield: i32,
    recharge: i32,
    total_spent: i32,
}

#[derive(Clone, Debug)]
struct Spell {
    cost: i32,
    damage: i32,
    heal: i32,
    recharge: i32,
    shield: i32,
    poison: i32,
}

fn main() {
    let spells = [
        Spell {
            cost: 53,
            damage: 4,
            heal: 0,
            recharge: 0,
            shield: 0,
            poison: 0,
        },
        Spell {
            cost: 73,
            damage: 2,
            heal: 2,
            recharge: 0,
            shield: 0,
            poison: 0,
        },
        Spell {
            cost: 113,
            damage: 0,
            heal: 0,
            recharge: 0,
            shield: 6,
            poison: 0,
        },
        Spell {
            cost: 173,
            damage: 0,
            heal: 0,
            recharge: 0,
            shield: 0,
            poison: 6,
        },
        Spell {
            cost: 229,
            damage: 0,
            heal: 0,
            recharge: 5,
            shield: 0,
            poison: 0,
        },
    ];

    let mut boss = Boss {
        hp: 0,
        damage: 0,
        poison: 0,
    };
    for line in fs::read_to_string("../inputs/day22.txt")
        .unwrap()
        .trim()
        .lines()
    {
        let parts = line.split(": ").collect::<Vec<_>>();
        if parts[0] == "Hit Points" {
            boss.hp = parts[1].parse().unwrap();
        } else {
            boss.damage = parts[1].parse().unwrap();
        }
    }

    let player = Player {
        hp: 50,
        mana: 500,
        shield: 0,
        recharge: 0,
        total_spent: 0,
    };

    let mut result = i32::MAX;
    play(&player, &boss, &mut result, &spells);

    println!("{}", result);
}

fn play(player: &Player, boss: &Boss, result: &mut i32, spells: &[Spell]) {
    player_turn(player, boss, result, spells);
}

fn player_turn(player: &Player, boss: &Boss, result: &mut i32, spells: &[Spell]) {
    let mut player = player.clone();
    let mut boss = boss.clone();

    if player.total_spent >= *result {
        return;
    }

    if player.recharge > 0 {
        player.mana += 101;
        player.recharge -= 1;
    }

    if player.shield > 0 {
        player.shield -= 1;
    }

    if boss.poison > 0 {
        boss.hp -= 3;
        boss.poison -= 1;
    }

    if boss.hp <= 0 {
        *result = (*result).min(player.total_spent);
        return;
    }

    for spell in spells {
        if boss.poison > 1 && spell.poison > 0 {
            continue;
        }

        if player.recharge > 1 && spell.recharge > 0 {
            continue;
        }

        if player.shield > 1 && spell.shield > 0 {
            continue;
        }

        if player.mana < spell.cost {
            continue;
        }

        let mut player = player.clone();
        let mut boss = boss.clone();

        player.mana -= spell.cost;
        player.total_spent += spell.cost;

        player.hp += spell.heal;
        boss.hp -= spell.damage;
        boss.poison = boss.poison.max(spell.poison);
        player.recharge = player.recharge.max(spell.recharge);
        player.shield = player.shield.max(spell.shield);

        boss_turn(&player, &boss, result, spells);
    }
}

fn boss_turn(player: &Player, boss: &Boss, result: &mut i32, spells: &[Spell]) {
    let mut player = player.clone();
    let mut boss = boss.clone();

    if player.recharge > 0 {
        player.mana += 101;
        player.recharge -= 1;
    }

    if player.shield > 0 {
        player.shield -= 1;
    }

    if boss.poison > 0 {
        boss.hp -= 3;
        boss.poison -= 1;
    }

    if boss.hp <= 0 {
        *result = (*result).min(player.total_spent);
        return;
    }

    player.hp -= 1.max(boss.damage - if player.shield > 0 { 7 } else { 0 });
    if player.hp > 0 {
        player_turn(&player, &boss, result, spells);
    }
}
