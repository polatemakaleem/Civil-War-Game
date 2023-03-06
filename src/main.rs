mod pools;
mod ents;
mod functions;

use functions::*;
use ents::player::*;
use ents::enemy::*;
use num::traits::Pow;
use pools::year_event::*;
use pools::turn_event::*;
use rand::prelude::*;

use crate::pools::enemy_event::enemy_turn;

const YEARS:u8 = 4;
const TURNS:u8 = 2; //subject to change

fn main(){
    println!("\n\t\t\t\tWelcome to the Civil War Game!");
    next("\t    Press enter to start!");
    let mut p:Player = Player { money:234000, morale:80, negotiated:false, army_size: 250000, intel:false, year:-1, blockade:false, health:0, draft:false}; //change money to fit time period
    let mut e = Enemy { money:115234, morale: 95, army_size:140326};
    game(&mut p, &mut e);

    println!("\n\n\n\t\t\t\t\t    GAME OVER!");

    println!("\n\n\t\t\t\t\tPlayer Stats:");
    println!("\n
            MONEY LEFT: ${}   PUBLIC MORALE: {}%   SOLDIERS LEFT: {}", p.money, p.morale, p.army_size);
    println!("\n\n\t\t\t\t\tEnemy Stats:");
    println!("\n
            MONEY LEFT: ${}   PUBLIC MORALE: {}%   SOLDIERS LEFT: {}", e.money, e.morale, e.army_size);
    
    next("\t   Press enter to see sources.");
    print_sources();
}

fn game(p: &mut Player, e: &mut Enemy) {

    for year in 0..=YEARS {
        p.year += 1;
        println!("\n\n\t\t\t\t\t    YEAR: {year}");
        event(p, e);
        next("\t  Enter anything to continue.");
        if year == 0 {continue;} // no turns for intro year
        if !p.check_army() {return}
        if !e.check_army() {return}
        for turn in 1..=TURNS {
            
            println!("\n\n\t\t\t\t\t  TURN: {turn}");
            println!("\n
            MONEY LEFT: ${}   PUBLIC MORALE: {}%   SOLDIERS LEFT: {}", p.money, p.morale, p.army_size);
            player_event(p, e);
            next("\t  Enter anything to continue.");
        }
        let mut rng = thread_rng();
        if e.morale <= 15 {
            p.subtract_army(rng.gen_range(10000..=25000));
        }
       
        p.disease_army(rng.gen_range(9.6..=10.15).pow(((p.health as f64)*(-0.25))+4.5) as i32);
        e.subtract_army(rng.gen_range(10.0..=10.75).pow(4.15) as i32);
        next("Enter anything to acknowledge the wasted lives.");
        if !e.check_army() {return;}

        for turn in 1..TURNS {
            println!("\n\n\t\t\t\t    CONFEDERACY'S TURN: {turn}");
            enemy_turn(p, e);
            next("\t  Enter anything to continue.");
        }
        
        if p.morale <= 35 {
            println!("\n
    Soldiers have become so discouraged from fighting that they are now starting to desert.
    Try to warm up their spirits however you can.");
            p.subtract_army(rng.gen_range(25000..=40000));
            next("Enter anything to acknowledge your incompetence.");
        }
        
        if !p.check_army() {return;}
    }
   
    println!("\n\n\t\t\t\t\t    YEAR: 5");
    print!("\n
    As the final year of your presidency term runs out, a new presidential election arises.
    ");
    if p.morale <= 50 {
        print!("People are sick and tired of fighting the South, so they do not elect you again.
    This was exactly what the Confederacy wants, and when the country is weakened by a new 
    president, they strike against the Union and end things once and for all.");
    } else{
        print!("People are still wanting for this war, so they reelect you as president to put down
    the Confederacy. The Confederacy knows that they would not be able to last much longer, so they
    end up surrendering to the Union. The North and South finally get reconnected, and the country
    continues as the United States of America.");
    }
    use std::io::Write;
    std::io::stdout().flush().unwrap(); //prints the buffer

}

fn print_sources() {
    println!("\n\nWORKS CITED:
    
    U.S. Department of the Interior. (2021, October 27). Facts. National Parks Service
        Retrieved February 23, 2023, from https://www.nps.gov/civilwar/facts.htm#:~:text=
        Enlistment%20strength%20for%20the%20Union
    
    American Battlefield Trust. (n.d.). Retrieved from https://www.battlefields.org
        /search?type%5B0%5D=battle&amp;type%5B1%5D=battle&amp;period%5B0%5D=71&amp;period
        %5B1%5D=71 
    
    Zataraga, M. (2015, April 14). The battle of roanoke island. National Parks Service.
        Retrieved 2023, from https://www.nps.gov/fora/learn/historyculture
        /battleofroanokeisland.htm 
    
    The battle of Lynchburg. Shenandoah Valley Battlefields National Historic District.
        (n.d.). Retrieved March 6, 2023, from https://www.shenandoahatwar.org
        /battle-of-lynchburg 
        ");
}