use yansi::{Paint, Color};

use rand::Rng;
use std::io;
use std::io::Write;

#[derive(Clone)]
enum CreatureType  { PLAYER, NPC }
#[derive(Clone)]
enum ItemType  { SPELL, OTHER }
#[derive(Debug, PartialEq, Eq)]
enum ActivityMode  { IDLE, FIGHT, ALL }
enum FightResult   { PLAYER_WINS, NPC_WINS, FIGHT_CONTINUES }
enum CommandResult { NONE, CONTINUE, BREAK, EXIT_GAME }

#[derive(Clone)]
struct Creature {
    ctype: CreatureType,
    name: String,
    description: String,
    location:u32,
    str: u32, dex: u32, int: u32, hp:i32, mhp:i32, xp:u32, defeat_xp:u32,
    inventory: Vec<Item>,
    spells: Vec<Item>
}
#[derive(Clone)]
struct Item {
    itype : ItemType,
    name: String,
    description : String,
    weight:u32, size:u32, 
    use_item: fn(&mut Creature,Option <&mut Creature>) -> CommandResult
}
#[derive(Clone)]
struct Location {
    id: u32,
    name: String,
    description:String,
    n:i32,ne:i32,e:i32,se:i32,s:i32,sw:i32,w:i32,nw:i32,u:i32,d:i32
}
struct Command {
    command:String,
    mode : ActivityMode,
    action: fn(Vec<&str>, Location, &mut Creature, &mut Option <&mut Creature>) -> CommandResult
}


fn main() {
    init();

    let action = prompt("What will you do? help(h)? ");
    println!("User entered {action}");
}

fn init() {
    println!("{}!", Paint::green("Starting text_rpg v1.0 ").bold());

    let mut player = create_player();
    add_player_spells(&mut player);
    add_player_items(&mut player);

}

fn create_player() -> Creature {
    let char_name = prompt("Please enter your characters name:");
    let mhp = rand::thread_rng().gen_range(10..=15);
    let mut player = Creature {
        ctype: CreatureType::PLAYER,
        name:char_name,
        description: "The player".to_string(),
        location:1,
        str:rand::thread_rng().gen_range(3..=18),
        dex:rand::thread_rng().gen_range(3..=18),
        int:rand::thread_rng().gen_range(3..=18),
        mhp:mhp,
        hp:mhp,
        xp:0,
        defeat_xp:0,
        inventory:vec![],
        spells:vec![]
   };
   
   player
}
fn add_player_spells(player: &mut Creature) {
}
fn add_player_items(player: &mut Creature) {
}

fn prompt(prompts:&str) -> String {
    print!("{prompts}");
    io::stdout().flush() ;

    let mut user_input = String::new();

    io::stdin().read_line(&mut user_input)
        .expect("Failed to read line");

    user_input.trim_end().to_string()
}

