pub struct Enemy {
    pub money:i32,
    pub morale:i8,
    pub army_size:i32,
}

impl Enemy {
    pub fn add_morale(&mut self, morale:i8){
        self.morale = std::cmp::min(self.morale + morale, 100);
    }
    pub fn subtract_morale(&mut self, morale:i8){
        self.morale = std::cmp::max(self.morale - morale, 0);
    }
    pub fn subtract_army(&mut self, soldiers:i32) {
        self.army_size = std::cmp::max(self.army_size - soldiers, 0);
    }
    pub fn pay_money(&mut self, cost:i32) -> bool {
        if cost > self.money {
            return false;
        } else {
            self.money -= cost;

            return true;
        }
    }
    pub fn check_army(&mut self) -> bool {
        if self.army_size <= 0 {
            println!("\n
    The Confederacy diminishes as you finish off the last portion of their army. They surrender. The Union takes the form 
    of the United States once again, this time with slavery being completely illegal. The war is won,
    you, Abraham Lincoln, will go down in history as one of the most influential figures that determined
    the fate of the United States.");
            return false;
        }
        return true;
    }
}