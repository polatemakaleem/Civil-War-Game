use crate::ents::enemy::*;
use crate::ents::player::*;
use crate::functions::*;
use crate::pools::turn_event::funds;
use rand::prelude::*;
pub fn event(p: &mut Player, e: &mut Enemy) {
    match p.year {
        0 => return intro(p),
        1 => return sumter_attack(p, e),
        2 => return stonewall_valley(p, e),
        3 => return gettysberg(p, e),
        4 => return lynchburg(p, e),
        _ => println!("lol!"),
    }
}

fn lynchburg(p: &mut Player, e: &mut Enemy) {
    println!("\n
    Union commanders and armies march towards the Southern states. These
    soldiers arrive at Lynchburg, Tennessee. Although their plans to take
    over the area gets easily thwarted, they get the chance to deal massive
    damage. Tennessee and the surrounding territories are where the majority
    of Southern farming takes place.");
    p.subtract_army(10515);
    e.subtract_army(2005);
    p.subtract_morale(10);
    println!("\n

    What are your orders?
    
        1. Destroy everything the soldiers can find
        2. Retreat as fast as possible");
    match menu(1, 2, Vec::new()) {
        1 => {
            println!("\n
    The soldiers trample and burn everything they find, leaving nothing left.
    The Confederates lose a lot of their farmland and resources. The Union
    leaves the area safely.");
            p.add_morale(65);
            e.subtract_morale(65);
            e.pay_money(e.money/2);
        }
        2 => {
            println!("\n
    You figure that the soldiers should retreat so they do not risk any
    further confrontation.");
        }
        _ => println!("")
    }
}

fn prepare_war(p: &mut Player) {
    println!("\n
    You believe that the South is going to start getting extremely violent.
    South Carolina and other states seceeded from the United States shortly after
    you were elected, forming the Confederacy. War becomes more and more likely.
    
    What should you prepare?
    
        1. Grow army
        2. Raise funds");
    
    match menu(1, 3, Vec::new()){
        1 => {println!("\n
    You decide to start growing your army. You start putting out posters asking 
    for volunteers. The public is a bit worried, but people join nonetheless.");
    p.add_army(25000);
    p.subtract_morale(5);   
    },
        
        2 => funds(p),

        _ => println!("impossibruh"),
    }
}


fn stonewall_valley(p: &mut Player, e: &mut Enemy) {
    println!("\n
    The Confederacy attacks Union forces under commanding eye of Ulysses S. Grant. It
    occurs at Shiloh, Tennessee. There are currently 63,000 Union soldiers stationed,
    while there is an estimated value of 40,000 Confederate troops.
    
    What are your orders?
        
        1. Fight back
        2. Retreat");
    
    match menu(1, 2, Vec::new()) {
        1 => {
            println!("\n
    General Grant leads his troops against the Confederate brigade. Although some
    lives were lost, the Confederacy felt the impact and fled from sight. Grant,
    knowing his troops were tired and unprepared to chase, did not follow. This
    was a much needed win for the Union.");
            p.subtract_army(13000);
            p.add_morale(5);
            e.subtract_army(11000);
        },
        2 => {
            println!("\n
    General Grant fled from sight, even though he could have easily overwhelmed
    the disadvantaged Confederacy. Minimal lives are lost, but there could
    have been a heftier blow to Confederate forces.");
    p.subtract_morale(10);
        }
        _ => println!("impossibruh"),
    }
}


fn sumter_attack(p: &mut Player, e: &mut Enemy) {
    println!("\n
    The Confederacy, pissed off, begins capturing Union (American) forts. One of the major ones,
    Fort Sumter, gets put under siege by Confederate gunfire and mortar shots. Your commander of
    this region, Robert Anderson, tries to hold out as long as he can for your orders.
    
    What are your orders?");

    println!("
    \t1. Hold out and wait for supplies
    \t2. Fight back
    \t3. Retreat");

    match menu(1, 3, Vec::new()) {
        1 => sumter_wait(p, e),
        2 => sumter_fight(p, e),
        3 => sumter_retreat(p),
        _ => println!("impossibruh"),
    }
    println!("\n
                                        THE WAR BEGINS!");                                   

}

fn sumter_retreat(p:&mut Player) {
    println!("\n
    Commander Anderson runs away. I mean, it was the right choice.
    It's just not very fun. Some soldiers get hurt on their way out,
    but none get mortally wounded. This ends up having no casualties.
    Congratulations, you took the lame way out.");
    p.subtract_morale(5);
}

fn sumter_fight(p: &mut Player, e: &mut Enemy) {
    println!("\n
    Commander Anderson fights back against the Confederacy, dealing as much damage as possible.
    Unfortunately, this plan didn't work out as well as you had first thought. Anderson dies
    via a blast from a mortar shell while trying to help wounded soldiers get out of danger.
    The nation suffers this massive blow right out of the gates. Many families mourn for the
    countless deaths that occurred. However, their sacrifices were not in vain. While on their
    last stand, they took out some Confederate soldiers. Look to the bright side, eh?");
    p.subtract_army(75000);
    p.subtract_morale(25);
    e.subtract_army(15000);
}

fn sumter_wait(p: &mut Player, e: &mut Enemy) {
    println!("\n
    Commander Anderson stands his ground, waiting for any reinforcements or supplies. The Confederacy
    continues to bomb out this fort, wounding many and ultimately preparing the fort for takeover.
    No supplies make it in time in order for the soldiers at the fort to even stand a chance. Fort
    Sumter falls, and some of the soldiers retreat and make it out.");
    p.subtract_army(50000);
    p.subtract_morale(15);
    e.add_morale(10);
}

fn intro(p: &mut Player) {

    println!("\n
    The year is 1860. You, Abraham Lincoln, just got elected president of the United States.
    Tensions are high, and slavery becomes a catalyst for violence within the country. 
    The industrial, abolitionist North versus the agrarian, slave-owning South. 
    There has been repeated attempts to settle matters, but both sides believe 
    that the other has some unknown control in the government.
    
    How can you tame the hatred between the factions?");
    
    println!("
    \t1. Negotiate further
    \t2. Declare North right
    \t3. Declare South right
    \t4. Prepare for War");

    match menu(1, 4, Vec::new() ) {
        1 => intro_negotiate(p),
        2 => declare_north_right(p),
        3 => declare_south_right(p),
        4 => prepare_war(p),
        _ => println!("impossibruh"),
    }

}

fn declare_south_right(p: &mut Player) {
    println!("\n
    This was very poorly timed. South Carolina just seceeded from the United States,
    other states following them. You just showed loyalty to the party that didn't 
    even put you on their ballots, and were so mad that they left the United States 
    after you were elected. You are left with the states that you showed disloyalty
    to. Understandably, they are confused and very angry.");
    p.subtract_morale(30);
}

fn declare_north_right(p: &mut Player) {
    println!("\n
    Well, even before you paraded your loyalty to the abolishonists, South Carolina
    had seceeded from the United States. They created the Confederacy,
    and you just provoked them. Luckily, this makes you look better in the eyes of
    the new Union.");
    p.add_morale(10);
}

fn intro_negotiate(p: &mut Player) {

    println!("\n
    You decide to start planning negotiation terms to try to even out the field for slave states and free states.
    However, while thinking of terms for this new pact, you receive news that South Carolina has seceeded
    from the United States. Many other states follow suit, forming the Confederacy.");

    println!("\n
    Do you still go through with this negotiation?
    \t1. Yes
    \t2. No");

    match menu(1, 2, Vec::new()) {
        1 => {println!("\n
    You think that the Confederacy may still be open for changes within the United States,
    and you make terms and conditions for both the North and South in an attempt to negotiate.
    You actually got some representatives from Southern States still in the United States, 
    and managed to bring very little peace to the table.");
    p.negotiated = true;},
        2 => {println!("\n
    You decide that it is too late for a negotiation, and instead start preparing for war.");},
        _ => println!("impossibruh"),
    }
}
fn gettysberg(p: &mut Player, e: &mut Enemy) {
    println!("\n
    As July of 1863 comes around, Confederate Commander Robert E. Lee makes it obvious
    that he is planning an swift attack northward to strike a heavy blow to the Union.
    He pushed onwards, and made it to Adams County, Pennslyvania. You command General
    Hooker to proceed and attack this 75,000 man army moving through Northern Virginia.
    However, Hooker feels like he would rather sit in camp and not attack.
    
    What do you do?
        
        1. Fire him
        2. Give motivation");
        
    let name;
    loop {
    match menu(1, 2, Vec::new()) {
        1 => {
            println!("\n
    You fire General Hooker and replace him with General George Gordon Meade, and he takes charge in this push.");
            name = "General George Meade".to_string();
            break;
        }
        2 => {
            let mut rng = thread_rng();
            if p.pay_money(rng.gen_range(15000..=25000)) {
               println!("\n
    You decide to give General Hooker some 'motivation'. He enjoys this motivation and pushes towards Lee's army.");
                name = "General Hooker".to_string();
                break;
            }
            else {
                println!("\n
    You do not have enough money to give General Hooker motivation.");
            }
        }
        _=> println!("impossibruh")
    }
    }
    println!("\n\t\t\t\t\t\t  DAY 1");
    next("\t   Enter anything to continue");
    println!("\n
    On July 1st, a Confederate division returns to Gettysburg (inside Adams County), and comes face to face with the
    Union army. There is going to be a nasty fight here.
    
    What are your orders?
        
        1. Slow down the Confederate troops until reinforcements arrive
        2. All out attack");
    match menu(1,2, Vec::new()) {
        1 => {
            println!("\n
    {name} attempts to slow down the Confederate army, at least until more soldiers arrive. As more and more corpses pile on,
    more reinforcements arrive. The plan somewhat worked out, and reinforcements are now gathering.");
            p.subtract_army(28185);
            e.subtract_army(9625);
            
        }
        2 => {
            println!("\n
    {name} prepares an all out attack on the Confederate army. He charges his men in, prepared for the worst. The worst happens.
    This massive army shreds through the Union army, but it does give the Union army a chance to kill some of the Confederate
    soldiers as well.");
            p.subtract_army(37579);
            e.subtract_army(12031);
            
        }
        _ => println!("")
    }
    p.subtract_morale(25);
    next("\t   Enter anything to continue.");

    println!("\n
    After fighting for a while, {name} notices the Confederate army advancing.
    They have no choice but to fall back. They retreat down to Cemetery Hill,
    which is somewhat south of Gettysburg.");

    println!("\n\t\t\t\t\t\t  DAY 2");
    next("\t  Enter anything to continue");
    println!("\n
    On the second day of battle, July 2nd, {name} and his army defends against Confederate armies on
    Cemetery Hill. In the afternoon, Lee launches a heavy assault on the Union. This assault causes
    massive bloodshed at multiple different places, generating many casualties. These assaults surround
    the Union forces.
    
    How are you going to defend these soldiers?
        
        1. Brute force through Confederate armies
        2. Cut a hole in the surroundings and escape");

    match menu(1, 2, Vec::new()) {
        1 => {
            println!("\n
    Although you have the higher numbers, the Confederacy moves quickly and with purpose. They start mowing down
    Union soldiers, making the corpses' blood trickle down the hills like rain off a roof. {name} only 
    has one choice, and that is to let the Confederacy take that part of land, and retreat elsewhere.
    The sun sets, but it does not mean peace and quiet on the war field.");
            p.subtract_army(50105);
            e.subtract_army(12031);
        }
        2 => {
            println!("\n
    {name} commands his forces to attack one specific portion of this assault.
    There is an opening in the corpses of the Confederacy. {name} 
    and his army rush through this opening and retreat, allowing the Confederacy
    to take over a portion of the land they were defending.");
            p.subtract_army(28185);
            e.subtract_army(12031);
        }
        _ => println!("")
    }
    p.subtract_morale(50);
    next("\t   Enter anything to continue.");

    println!("\n\t\t\t\t\t\t  DAY 3");
    println!("\n
    Waking up bright and early, both parties decide that it is the opportune time to take each others' land.
    The Union attempts to take back the land they lost, while the Confederacy means to take the rest of
    the Union's land. The Confederacy sends George E. Pickett, and his 'army' of about 6,250 soldiers through
    farmland and towards the Union territory. Union forces, amounting to around 12,500 soldiers, are awaiting
    for soldiers to arrive to the highest point of the ridge.");
    next("\t   Enter anything to continue.");
    println!("\n
    When the Confederacy eventually arrives, it is a total bloodbath. The Confederacy's army has about a 
    60% casualty percentage. Lee takes his army and runs away on July 4th, which is very symbolic.");
    p.subtract_army(25665);
    e.subtract_army(21049);
    next("\t   Enter anything to continue.");
    println!("\n
    In November of 1863, you decide to give a speech in Gettysberg about the battle. Here's the script:
    
    'Four score and seven years ago our fathers brought forth on this continent a new nation, conceived
    in liberty, and dedicated to the proposition that all men are created equal.

    Now we are engaged in a great civil war, testing whether that nation, or any nation so conceived and
    so dedicated, can long endure. We are met on a great battlefield of that war. We have come to dedicate
    a portion of that field as a final resting place for those who here gave their lives that that nation
    might live. It is altogether fitting and proper that we should do this.
    
    But in a larger sense we cannot dedicate, we cannot consecrate, we cannot hallow this ground. The brave
    men, living and dead, who struggled here have consecrated it, far above our poor power to add or detract.
    The world will little note, nor long remember, what we say here, but it can never forget what they did here.
    It is for us the living, rather, to be dedicated here to the unfinished work which they who fought here
    have thus far so nobly advanced. It is rather for us to be here dedicated to the great task remaining
    before us,that from these honored dead we take increased devotion to that cause for which they gave the
    last full measure of devotion, that we here highly resolve that these dead shall not have died in vain,
    that this nation, under God, shall have a new birth of freedom, and that government of the people, by the
    people, for the people, shall not perish from the earth.");
    p.add_morale(95);
    e.subtract_morale(50);
}