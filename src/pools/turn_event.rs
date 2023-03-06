use crate::{ents::{player::Player, enemy::*}, functions::*};
use rand::prelude::*;

pub fn player_event(p: &mut Player, e: &mut Enemy) {
    let mut blacklist:Vec<u8> = Vec::new();
    print!("\n
    1. Attack the Confederacy
    2. Promote the War
    3. Raise funds
    4. ");
    if p.intel {
        print!("ACTIVE
    5. ");
        blacklist.push(4);
    }
    else {
        print!("Intel Gathering
    5. ");
    }
    if p.blockade {
        print!("ACTIVE");
        blacklist.push(5);
    }
    else {
        print!("Form a Blockade");
    }
    print!("
    6. ");
    if p.health != 2 {
        print!("Improve Living Conditions for Soldiers");
    }
    else {
        print!("MAXED");
    }
    use std::io::Write;
    std::io::stdout().flush().unwrap(); //prints the buffer
    match menu(1, 6, blacklist){
        1 => attack(p, e),
        2 => promote(p),
        3 => funds(p),
        4 => intel(p),
        5 => blockade(p),
        6 => health_conditions(p),
        _ => println!("impossibruh"),
    }
}

pub fn funds(p: &mut Player) {
    println!("\n
    You decide to collect funds for the war effort. Although people don't really like
    giving more money to the government, they understand that the cause is worthy.");
    let mut rng = thread_rng();
    let num = rng.gen_range(47512..=79054);
    p.money += num;
    println!("\n
                                         +{num} Money");
}


fn year_2(p: &mut Player, e: &mut Enemy) { // player picks battle for year 2
    println!("\n
    1. Fort Henry, Tennessee
    2. Roanoke Island, North Carolina
    3. Kernstown, Virginia");
    match menu(1, 3, Vec::new()) {
        1 => fort_henry(p, e),
        2 => roanoke_island(p, e),
        3 => kernstown(p, e),
        _ => println!("impossibruh"),
    }
}

fn kernstown(p: &mut Player, e: &mut Enemy) {
    println!("\n
    While Confederate General Thomas Stonewall Jackson was cleaning up an
    army division sent to capture him, reinforcements arrived. He tied up the
    loose ends, and put the new reinforcements under siege. Current Union General 
    James Shields is leading these Union reinforcements. Unfortunately, he gets wounded
    during gunfire. The new General is Nathan Kimball, currently stuck under Confederate
    gunfire.

    What should Kimball do?
        1. Cut a hole in the siege and retreat
        2. Hold out
    ");
    match menu(1, 2, Vec::new()) {
        1 => {println!("\n
    Kimball attempts to rush a specific part of the siege to cut a hole and escape through it.
    He commands his men to take shots at the people surrounding them, and then to rush out when 
    they are all dead or wounded. 
    
    However, the rush attempts fail and a huge chunk of this division gets mowed down by 
    the Confederates. Kimball sees his only chance at escaping the siege get shot down
    in front of him. He surrenders Stonewall Jackson.");
            p.subtract_army(75000);
            p.subtract_morale(40);
            e.add_morale(40);
            e.subtract_army(1500);
        },
        2 => {println!("\n
    Kimball tries to just hold out as long as possible for reinforcements. After a few hours,
    Stonewall Jackson's men pulled back. Kimball could see Jackson trying to convince them
    to stay, but to no avail. Kimball thinks that they ran out of supplies due to them just
    coming from another fight.
    
    Although you won the fight, people are still worried that the Confederates came this
    close to the nation's capital.");
            p.subtract_army(1500);
            p.subtract_morale(15);
            e.subtract_army(30000);
            e.subtract_morale(15);
        },
        _ => println!("impossibruh"),
    }
}

fn year_3(p: &mut Player, e: &mut Enemy) { // player picks battle for year 3
    println!("\n
    1. Prince William County, Virginia
    2. Chickamauga, Georgia");
    match menu(1, 2, Vec::new()){
        1 => william_county(p, e),
        2 => chickamauga(p, e),
        _ => println!("impossibruh"),
    }
}

fn year_4(p: &mut Player, e: &mut Enemy) {
    println!("\n
    1. Nashville, Tennessee
    2. Spring Hill, Tennessee");
    match menu(1, 2, Vec::new()) {
        1 => nashville(p, e),
        2 => spring_hill(p, e),
        _ => println!("")
    }
}

fn nashville(p: &mut Player, e: &mut Enemy) {
    println!("\n
    Confederate forces, led by John Bell Hood, are convinced that they will
    take over Nashville, Tennessee. The armies are seen by George Thomas, the
    overall commander of Tennessee.
    
    What are your orders?
    
        1. Distract the enemy and strike
        2. Defend the current area");
    
    match menu(1, 2, Vec::new()) {
        1 => {
            println!("\n
    Although the distraction doesn't work for long, Hood gets confused and throttled.
    His army is massively reduced, but he is still pushing through. The Union keeps on
    picking off soldiers and thinning the army, but Hood doesn't seem to care. 
    Eventually he retreats.");
            p.subtract_army(13914);
            p.add_morale(22);
            e.subtract_army(28065);
            e.subtract_morale(22);
        }
        2 => {
            println!("\n
    Despite meeting heavy resistance, Hood keeps on pushing through. The Confederate
    soldiers eventually breaches the outer defense of the Union. George Thomas falls
    back a small bit, and recreates the formation. Confederate soldiers kept on pushing,
    even if it was massively reducing their numbers.
    
    What are your orders?
        
        1. Keep on falling back
        2. Retreat");
        match menu(1, 2, Vec::new()) {
            1 => {
                println!("\n
    After hours of falling back and laying on soldiers, Hood finally retreats. The
    battlefield is filled with corpses of mosly Confederate soldiers.");
                p.subtract_army(27828);
                p.add_morale(35);
                e.subtract_morale(35);
                e.subtract_army(37098);
            }
            2 => {
                println!("\n
    George Thomas retreats from the defensive position, and Hood's reckless strategy
    works out. The Confederacy gains control over areas in Tennessee.");
                p.subtract_morale(20);
                e.add_morale(20);
            }
            _ => println!("")
        }
        }
        _ => println!("")
    }
}

fn spring_hill(p: &mut Player, e: &mut Enemy) {
    println!("\n
    Union Commander John M. Shofield has defended against Confederate pushes in Tennessee
    before, but now he faces a particularly aggressive Confederate Commander, John Hood. 
    The Confederate cavalry meets the Union cavalry at Spring Hill and engages combat.
    Overall, the fight does not have a clear winner, as both sides stay at around the 
    same point. The battle has no clear winner, but when night comes around, the
    Confederacy gives up and leaves.");
    p.subtract_army(12500);
    e.subtract_army(5847);
}

fn chickamauga(p:&mut Player, e: &mut Enemy) {
    println!("\n
    Confederate armies spot a Union army going North through Georgia. Union General
    William Rosecrans assumes that the Confederate soldiers would flee Southward
    if he approaches them. With that in mind, he split his men into 3 groups to
    split up and attack from multiple angles. However, the Confederate soldiers
    did not flee. They instead combined forces at LaFayette, waiting for 
    reinforcements. One of Rosecrans's groups is approaching these soldiers.
    
    What should Rosecrans do?
    
        1. Combine forces around LaFayette and defend
        2. Flee");
    match menu(1, 2, Vec::new()) {
        1 => {
            println!("\n
    Rosecrans gets his separated divisions back together in an attempt to defend
    Chickamagua from Confederate control. The Confederacy immediately begins an 
    assault on the defense, but it is seen in advance and throttled. The battle
    temporarily comes to a close as the sun sets.");
            p.subtract_army(33688);
            e.subtract_army(25624);
            next("\t   Enter anything to continue");
            println!("\n
    What are your orders?
    
        1. Defend for longer
        2. Flee");
            match menu(1, 2, Vec::new()) {
                1 => {
                    println!("\n
    Early the next day, the fighting recommenced. The Confederacy starts pushing
    for control over the area, and the Union tries to hold out. For a while, the Union
    was doing very well, until Confederate reinforcements arrived. Once the Confederacy
    had an advantage in numbers, the Union defenses fall.");
                    p.subtract_army(33688);
                    e.subtract_army(25624);
                    next("\t   Enter anything to continue");
                    println!("\n
    What are your orders?
    
        1. Flee");
                    match menu(1, 1, Vec::new()) {
                        1 => {
                            println!("\n
    Rosecrans runs away while taking heavy casualties. The Confederacy now has power
    over this area.");
                            p.subtract_army(32000);
                            p.subtract_morale(25);
                            e.add_morale(25);
                            return;
                        }
                        _ => println!("")
                    }
                }
                2 => {
                    println!("\n
    Rosecrans and his divisions run away. Some further casualties occur, and the Confederacy
    takes over the area.");
                    p.subtract_army(15924);
                    return;
                }
                _ => println!("")
            }
        }
        2 => {
            println!("\n
    Rosecrans and his divisions run away. No casualties occur, but the Confederacy
    takes over Chickamauga and the surrounding areas.");
            return;
        }
        _ => println!("impossibruh")
    }
}


fn attack(p: &mut Player, e: &mut Enemy) {
    let mut rng = thread_rng();
    if !p.pay_money(rng.gen_range(50000..=125000)) {
        println!("\n
    You figure you do not have enough money to form and carry out a good plan.");
    return;
    }
    print!("\n
    1. Attack!");
    let mut blacklist = Vec::new();
    if p.intel {
        print!("
    2. Informed attack")
    }
    else {
        print!("
    2. LOCKED");
        blacklist.push(2);
    }
    use std::io::Write;
    std::io::stdout().flush().unwrap(); //prints the buffer

    match menu(1, 2, blacklist) {
         1 => {
            println!("\n
    Where would you like to attack?");
            
            match p.year {
                1 => year_1(p, e),
                2 => year_2(p, e),
                3 => year_3(p, e),
                4 => year_4(p, e),
                _ => println!("impossombruh"),
            }
         }
         2 => {
            println!("\n
    You launch a surprise attack with gained inside information on the Confederacy.
    You intelligently plan out a surprise attack on the Confederacy, dealing massive
    damage while taking very little of your own.");
            p.subtract_army(10000);
            e.subtract_army(66667);
            p.intel = false;
         }
        _ => println!("impossibruh"),
    }
}

fn promote(p: &mut Player) {
    print!("\n
    You promote the war. You expect these advertisements to increase public morale
    and hopefully gather more volunteers for the war effort. ");
    if p.army_size <= 150000 && !p.draft {
        print!("You also have the idea
    of forcing citizens to join the Union army. You create what is now known
    as the Enrollment Act, the first national draft.
    
    From now on you will get more soldiers from promoting due to drafts.");
        p.draft = true;
    }
    print!("\n");
    use std::io::Write;
    std::io::stdout().flush().unwrap(); //prints the buffer
    let mut rng = thread_rng();
    p.add_morale(rng.gen_range(20..=35));
    let mut num = rng.gen_range(50000..=65000);
    if p.draft {num = (num as f32 * 1.5) as i32;}
    p.add_army(num);
}

fn intel(p: &mut Player) {
    let mut rng = thread_rng();
    print!("\n
    You attempt to gain intelligence on the Confederacy.");
    match rng.gen_range(1..=3) {
        1 => {
            print!("
    You send detective and spy Allan Pinkerton to try to get information.");
        },
        2 => {
            print!("
    You command spy-ring owner Elizabeth Van Lew to investigate the Confederate army.");
        },
        3 => {
            print!("
    You refer to your personal intelligence operative, William Lloyd, to give you information.");
        },
       _ => println!("impossibruh"),
    }
    next("\t  Enter anything to continue");
    use std::io::Write;
   std::io::stdout().flush().unwrap(); //prints the buffer
    if rng.gen_range(1..=3) == 1 {
        println!("\n
    The source comes through! You gain confidential intelligence on the Confederacy!");
    p.intel = true;
    } else {
        println!("\n
    The source fails. Sadly, no new information is gained.");
    }
}

fn william_county(p: &mut Player, e: &mut Enemy) {
    println!("\n
    Confederate Lieutenant A. P. Hill spotted the Union army While they were tracking down Lee's army after Gettysburg.
    Hill attacked immediately, without giving much thought into intelligence gathering. The Union general,
    K. Warren, currently has the disadvantage in terms of army size.
    
    What are your orders?
    
        1. Hide behind buildings and attack when the time is right
        2. Attack headfirst
        3. Retreat");
    match menu(1, 3, Vec::new()) {
        1 => {
            println!("\n
    Warren and his forces hide behind some buildings, awaiting the Confederate soldiers. When they appear, they are 
    in for a surprise. The Confederacy quickly surrenders, and the Union gets access to some artillery. The artillery
    are used to bomb out the rest of the army, quickly resolving the battle.");
            p.subtract_army(16104);
            p.add_morale(20);
            e.subtract_army(22494);
            e.subtract_morale(20);
        }
        2 => {
            println!("\n
    Warren and his underwhelming forces charge headfirst into the army with superior numbers. Smart move!");
            p.subtract_army(48312);
            p.subtract_morale(20);
            e.subtract_army(7498);
            e.add_morale(20);
        }
        3 => {
            println!("\n
    Warren and his forces retreat, not wanting to take chances with a superior-numbered group of soldiers.
    This works out well, and the Confederate forces lose Warren as he treks into the forest, losing 
    the track of Lee.");
        }
        _ => println!("")
    }
}

fn philippi(p: &mut Player, e: &mut Enemy) {
    println!("\n
    General George B. McClellan takes charge on this attack. He commanded his forces
    to surround the town of Philippia, and to start attacking the Confederate army
    at the sound of a pistol shot. A woman saw the Union converging, and tried to
    warn the Confederacy. This ended in a premature gunshot by the Confederacy,
    confusing the Union army. However, even with this early attack, the Union
    still sent the Confederacy running southward.");
    p.subtract_army(666);
    p.add_morale(5);
    e.subtract_army(1500);
    e.subtract_morale(5);
}


fn year_1(p:&mut Player, e: &mut Enemy) { // player picks battle for year 1
    println!("\n
    1. Barbour, West Virginia
    2. Manassas, Virginia");
    match menu(1, 2, Vec::new()) {
        1 => philippi(p, e),
        2 => first_bull_run(p, e),
        _ => println!("impossibruh"),
    }
}


fn first_bull_run(p: &mut Player, e: &mut Enemy) {
    println!("\n
    Irvin McDowell takes charge of this operation. He plans to take out Beauregard's unit
    before reinforcements come. He knows that this location contains a railroad that can 
    spread supplies to the Confederacy, so he plans to fight hard and quick to reduce
    the amount of soldiers he fights at once.");
    next("\t   Enter anything to continue");
    println!("\n
    On July 17th, he carries out his plan of quickly taking care of Beauregard's unit.
    However, the fight was more inconclusive than he had first hoped, and the
    reinforcements were quickly coming.");
    next("\t   Enter anything to continue");
    println!("\n
    On July 20th, reinforcements arrive. From here on, the Union starts to fall.
    Their artillery is not effective enough, and the Confederacy can see through
    their pushes. Eventually, the Confederacy's units pushes the Union back.");
    p.subtract_army(50896);
    p.subtract_morale(20);
    e.subtract_army(8629);
    e.add_morale(20);
}


fn blockade(p:&mut Player) {
    let mut rng = thread_rng();
    if p.pay_money(rng.gen_range(50000..=65000)) {
        println!("\n
    You form a blockade around Southern ports, trying to 
    reduce the amount of money they can make, and the 
    supplies they can import.");
        p.blockade = true;
    } else {
        println!("\n
    You crunch out the numbers and realize that you cannot
    afford to create a blockade. You abandon the idea and 
    look for something else to do.");
    }
}

fn health_conditions(p: &mut Player) {
    println!("\n
    You decide to improve health conditions for soldiers, hopefully prolonging
    more lives to have a bigger army, and thus have more of an advantage.");
    let mut rng = thread_rng();
    if p.pay_money(rng.gen_range(25000..=50000)) {
        if p.health == 0 {
            println!("\n
    You provide better living conditions, which include:
        - Better supply of food
        - More effective medicine supplied");
            p.health += 1;
    } else if p.health == 1 {
            println!("\n
    You provide better living conditions, which include:
        - More available nursing facilities, with staff
        - Better overall living conditions to reduce chances of disease");
            p.health += 1;
        }
    } else {
        println!("\n
    However, you do not have enough money to support this expenditure.
    You keep this idea on hold in case you have enough money in the future.");
    }
}


fn fort_henry(p: &mut Player, e: &mut Enemy) {
    println!("\n
    Fort Henry, a Confederate fort in Tennessee, is currently blocking
    a water route to get supplies more into Southern land. You give orders to General Grant
    and General Foote, telling them to take Fort Henry and any other forts currently blocking the path.
    
    What are your orders?
        1. Sneak attack
        2. Attack by water");
    match menu(1, 2, Vec::new()) {
        1 => {
            println!("\n
    You tell the generals to sneakily bombard the fort, hopefully causing its downfall. However,
    the Confederate troops stationed at Fort Henry catch the attack. The surprised Union army
    quickly starts taking damage, forcing the generals to flee.");
            p.subtract_army(52500);
            p.subtract_morale(25);
            e.subtract_army(10000);
            e.add_morale(15);
        }
        2 => {
            println!("\n
    The Union ships devastate Fort Henry, causing Confederate General Lloyd Tilghman to surrender.
    The fort is taken, and supplies can easily be moved to the front lines.");
            p.subtract_army(10000);
            p.add_morale(15);
            e.subtract_army(21500);
            e.subtract_morale(25);
        }
        _ => println!("impossibruh"),
    }
}

fn roanoke_island(p: &mut Player, e: &mut Enemy) {
    println!("\n
    You tell General Ambrose Burnside to attack east North Carolina. He plans to use
    the Outer Banks for the invasion. He has a fleet of 66 ships, and decides to send 
    them out to sea in Maryland. While on the trip from Maryland to North Carolina,
    some ships were lost in storm. However, there was still an impressive fleet
    by the time they arrived in Stumpy Point, North Carolina. From here, they could
    view a Confederate army and navy on Roanoke Island.
    
    What are your orders?
        1. Devise an attack
        2. Barge in with your superior navy");
    
    match menu(1, 2, Vec::new()) {
        1 => {
            println!("\n
    General Burnside decides to scout out enemy movements and defenses before attacking.
    He determines the Confederate weaknesses and ends up crafting a plan he believes
    will work out. 
    
    His army arrived on the island and quickly worked through all the defenses the 
    Confederates had with relative ease. They pushed through Roanoke Island, until
    the Confederate army eventually surrendered.");
            p.subtract_army(5000);
            p.add_morale(25);
            e.subtract_army(17540);
            e.subtract_morale(15);
        }
        2 => {
            println!("\n
    General Burnside stupidly barges in without a plan. He didn't expect the 
    three-gun battery and many ships. Whoops.");
            p.subtract_army(20000);
            p.subtract_morale(15);
            e.subtract_army(3000);
            e.add_morale(15);
        }
        _ => println!("impossibruh"),
    }
}