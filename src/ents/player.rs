use crate::functions::menu;

pub struct Player {
    pub money:u32,
    pub morale:i16,
    pub negotiated:bool,
    pub army_size:i32,
    pub intel:bool,
    pub year:i8,
    pub blockade:bool,
    pub health:u8,
    pub draft:bool,
}
impl Player {
	pub fn add_morale(&mut self, morale:i16){
        self.morale = std::cmp::min(self.morale + morale, 100);
        println!("
                                      +{morale}% Public Morale");
    }
    pub fn subtract_morale(&mut self, morale:i16){
        self.morale = std::cmp::max(self.morale - morale, 0);
        println!("
                                      -{morale}% Public Morale");
    }
    pub fn subtract_army(&mut self, soldiers:i32) {
        self.army_size = std::cmp::max( self.army_size - soldiers , 0);
        println!("
                                      -{soldiers} Soldiers");
    }
    pub fn add_army(&mut self, soldiers:i32) {
        self.army_size += soldiers;
        println!("
                                      +{soldiers} Soldiers");
    }
    pub fn pay_money(&mut self, cost:u32) -> bool {
        if cost > self.money {
            return false;
        } else {
            self.money -= cost;
            println!("
                                        -{cost} Money");
            return true;
        }
    }
    pub fn disease_army(&mut self, amount:i32) {
        self.army_size = std::cmp::max( self.army_size - amount , 0);
        println!("
                              {amount} soldiers died from disease.");
    }
    pub fn check_army(&mut self) -> bool {
        if self.army_size <= 0 { // change if negotiated
            if !self.negotiated {
            println!("\n
    You no longer have any soldiers in your army. The Confederacy launches one final attack
    on the Union, and no amount of random people you throw at them can stop them.
    Your country falls as slavery becomes embedded in the new United States.");
           return false;
            }
            println!("\n
    You no longer have any soldiers in your army. However, the Confederate army holds out an olive branch.
    You decided to negotiate with the South even though they were causing chaos and damage to citizens.
    They agree to reform the United States, but with extra conditions for their own livelihood.
    Such as:
        - Slavery is legal everywhere
        - States rights are preferred over federal law
        - Everyone is a mandatory reporter for runaway slaves
        - Slaves keep the same rights that they had in the South before the war
    
    Do you agree to this proposition?
        1. Yes
        2. No");
        match menu(1, 2, Vec::new()){
            1 => println!("\n
    You accept the proposition, hoping to be able to fight for another day. The Union and the Confederacy merge
    back into the United States, but this time with even more tension as the North are abolitionists that are forced
    to tend to slaves. The tension might be better than fully Southerners, though. It means that abolition
    still stands a chance."),
            2 => println!("\n
    The Confederation feels insulted that you didn't take their proposition. They gave you a chance, and you blew it.
    They warn you of an incoming invasion on the capital, and since you do not have enough soldiers to come even close to
    stopping their push, the capital, and thus the Union, falls to the Confederation. The new United States
    is formed out of the Confederation, and they make their own laws and rules."),
            _ => println!("impossibruh"),
        }
            return false;
        }
        return true;
    }
}