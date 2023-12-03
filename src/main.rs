use yansi::{Paint, Color};

use rand::Rng;
use std::io;
use std::io::Write;
use std::collections::HashMap;

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
    class : String,
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

struct Game {
    commands:Vec<Command>,
    locations :Vec<Location>,
    player : Creature
}


fn main() {
    let mut player = create_player();
    let mut game = Game{ commands :vec![], locations : vec![], player:player };
    init(&mut game);

    let action = prompt("What will you do? help(h)? ");
    println!("User entered {action}");

    let action_args:Vec<&str> = action.split(" ").collect();
    let cmd = game.commands.iter().find(|c| {
        (matches!(c.mode,ActivityMode::ALL) ) && c.command == action_args[0]
    });
    let res : CommandResult;

    if !matches!(cmd,None) {
        let mut mon:Option<&mut Creature> = None;
        let mut cloc = game.locations.iter().find(|l| l.id == game.player.location).unwrap();
        res = (cmd.unwrap().action)(action_args, Location::clone(cloc),&mut game.player,&mut mon);
    } else { 
        println!("Not sure what you mean there?")
    }

}

fn init(game:&mut Game) {
    add_player_spells(&mut game.player);
    add_player_items(&mut game.player);

    add_game_locations(game);
    add_game_commands(game);
}


fn add_game_locations(game: &mut Game) {
    game.locations.push(Location{id:1, name:"The Start".to_string(), description:"You are at the start of the dungeon".to_string(), n:0,ne:0,e:0,se:0,s:2,sw:0,w:0,nw:0,u:0,d:0});
    game.locations.push(Location{id:2, name:"corridor".to_string(),  description:"Its dank and dark and hard to see".to_string(), n:1,ne:0,e:0,se:0,s:0,sw:0,w:0,nw:0,u:0,d:0});

}

fn add_game_commands(game: &mut Game) {
   game.commands.push(Command { command:"h".to_string(), mode:ActivityMode::ALL, action: |action_args,cloc, player, mon| {
       println!("Some real help it is then");
       println!("h - help, show this text");
       CommandResult::CONTINUE
    }});

}

fn create_player() -> Creature {
    println!("{}!", Paint::green("Starting text_rpg v1.0 ").bold());
    let char_name = prompt("Please enter your characters name:");
    let mhp = rand::thread_rng().gen_range(10..=15);
    let mut player = Creature {
        ctype: CreatureType::PLAYER,
        name:char_name, description: "The player".to_string(), location:1,
        str:rand::thread_rng().gen_range(3..=18),
        dex:rand::thread_rng().gen_range(3..=18),
        int:rand::thread_rng().gen_range(3..=18),
        mhp:mhp, hp:mhp, xp:0, defeat_xp:0,
        inventory:vec![],
        spells:vec![]
   };
   
   player
}
fn add_player_spells(player: &mut Creature) {
    let mut spell_defs: HashMap<& str,Item>=HashMap::new();

    spell_defs.insert("spell.magicmissle", Item { itype:ItemType::SPELL, class:"spell".to_string(), name:"magicmissle".to_string(), description:"Magic Missle".to_string(),weight:5,size:5,
        use_item : |player, mon| {
            let mut res = CommandResult::CONTINUE;

            println!("Cast a spell, magic missile!");

            res
    } } );

    player.spells.push(Item::clone((spell_defs.get("spell.magicmissle").unwrap())));
}
fn add_player_items(player: &mut Creature) {
    let mut item_defs: HashMap<& str, Item>=HashMap::new();
    //let mut item_defs=HashMap::new();
 
    item_defs.insert("potion.healing",Item { itype:ItemType::OTHER, class:"potion".to_string(), name:"healing".to_string(),description:"A potion with a label Healing".to_string(), weight:5,size:5,
        use_item: |player, monster| {
            match monster {
                Some(monster) => { println!("Monster passed"); }
                None => { println!("No monster"); }
            }
            player.hp=player.hp+3;

            CommandResult::CONTINUE
        } }
    );

    player.inventory.push(Item::clone((item_defs.get("potion.healing").unwrap())));

}

fn prompt(prompts:&str) -> String {
    print!("{prompts}");
    io::stdout().flush() ;

    let mut user_input = String::new();

    io::stdin().read_line(&mut user_input)
        .expect("Failed to read line");

    user_input.trim_end().to_string()
}

