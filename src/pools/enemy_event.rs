use crate::{ents::{player::*, enemy::*}, functions::*};
use rand::prelude::*;

pub fn enemy_turn(p: &mut Player, e: &mut Enemy) {
    break_blockade(p, e);
    if !grow_army(p, e) { //if it doesnt raise funds
        if !raise_funds(p, e) { //if it doesnt grow army
            attack_player(p, e);
        }
    }
}

fn grow_army(p: &mut Player, e: &mut Enemy) -> bool {
    if e.army_size >= 50000 || p.army_size <= 100000 {
        return false;
    }
    let mut rng = thread_rng();
    println!("\n
    The Confederacy grows its army.");
    e.army_size += rng.gen_range(30000..=50000);
    e.pay_money(rng.gen_range(15000..=40000));

    return true;
}

fn attack_player(p: &mut Player, e: &mut Enemy) {
    let mut rng = thread_rng();
    println!("\n
    The Confederacy attacks!");
    let num = rng.gen_range(45000..=90000);
    p.subtract_army(num);
    p.subtract_morale(10);
    e.subtract_army(num/10);
    e.add_morale(10);   
}

fn raise_funds(p:&mut Player, e: &mut Enemy) -> bool {
    if p.army_size <= 100000 || e.money >= 40000 {
        return false;
    }
    println!("\n
    The Confederacy raises funds for the war.");
    let mut rng = thread_rng();
    e.money += rng.gen_range(35000..=60000);
    e.subtract_morale(5);
    return true;
}

fn break_blockade(p:&mut Player, e: &mut Enemy) {
    if p.blockade {
        let mut rng = thread_rng();
        println!("\n
    The Confederacy attempts to remove your blockade.");
    next("\t   Enter anything to continue");
    if !e.pay_money(rng.gen_range(25000..=30000)){
        println!("\n
    They can't afford it!");
        return;
    }
        if rng.gen_range(1..=2) == 1 {
            println!("\n
    The Confederacy successfully removes your blockade, removing its effects.");
            p.blockade = false;
        } else {
            println!("\n
    The Confederacy unsuccessfully removes your blockade.");
        }
    next("\t   Enter anything to continue");
    }
}