extern crate tcod;
extern crate rand;
use std::cmp::{PartialEq, Eq};
use std::ops::{Add};
use std::collections::{HashMap};
use std::rc::Rc;
use std::default::{Default};
use std::cell::RefCell;
use tcod::{BackgroundFlag};
use tcod::console::{Console, Root, FontLayout, FontType};
use tcod::input::{Key};
use tcod::input::KeyCode::{Escape, Char};
use rand::random;


trait Drawable {
    fn draw(&self, &mut Root);
}

trait Updates {
    fn update<'a>(&'a mut self, Key, &'a HashMap<Location, &'a Drawable>, &'a Universe) -> ConflictHandler;
}

enum ConflictType {
    Nothing,
    Occupied,
}

impl Default for ConflictType {
    fn default() -> Self {
        ConflictType::Nothing
    }
}

// #[derive(Default)]
struct ConflictHandler<'a> {
    kind: ConflictType,
    loc: Location,
    instigator: &'a Drawable,
    parties: Vec<&'a Drawable>,
}

// impl Default for ConflictHandler {
//     fn default() -> ConflictHandler {
//         ConflictHandler {
//             kind: ConflictType::Nothing, 
//             loc: Location { x : Default::default(), y : Default::default() },
//             parties: Vec::new(),
//         }
//     }
// }

struct Universe {
    c_level: usize,
    exit: bool,
    save: bool,
    window_bounds: Bounds,
    clock: i32,
}

impl Universe {
    fn time(&mut self) {
        self.clock += 1;
    }
}

struct Level<'a> {
    // num: i32,
    // monsters: Vec<Rc<RefCell<Monster>>>,
    monsters: Vec<Monster>,
    map: HashMap<Location, &'a Drawable>,
}

impl <'a> Level<'a> {
    // fn add_monster(&mut self, monster: Rc<RefCell<Monster>>) {
    fn add_monster(&'a mut self, monster: Monster) {
        let loc = monster.loc;
        self.monsters.push(monster);
        self.map.insert(loc, self.monsters.last().unwrap());
    }
    // fn update_monsters(&mut self, keypress: Key, player: &Player, universe: &Universe) -> ConflictHandler {
    //     for monster in self.monsters.iter_mut() {
    //     if monster.is_friendly {
    //         if monster.is_tame {
    //             monster.random_move(&mut self, universe);
    //         } else {
    //             monster.random_move(&mut self, universe);
    //         }
    //     } else {
    //         monster.random_move(&self, universe);
    //     }

    //     }
    // ConflictHandler }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Default)]
struct Location {
    x: i32,
    y: i32,
}

impl Location {
    fn chg(&self, chg: &Location) -> Location {
        Location { x : self.x + chg.x, y : self.y + chg.y }
    }
}

// impl Add for Location {
//     type Output = Location;

//     fn add(self, _rhs: Location) -> Location {


// impl PartialEq for Location {
//     fn eq(&self, other: &Location) -> bool {
//         self.x == other.x && self.y == other.y
//     }
// }

struct Bounds {
    min: Location,
    max: Location,
}

impl Bounds {
    fn chk_inside(&self, obj: Location) -> bool {
        obj.x >= self.min.x && obj.x <= self.max.x && obj.y >= self.min.y && obj.y <= self.max.y
    }
}

struct Player {
    loc: Location,
    sym: char,
}

impl Drawable for Player {
    fn draw(&self, con: &mut Root) {
        con.put_char(self.loc.x as i32, self.loc.y as i32, self.sym, BackgroundFlag::Set);
    }
}

impl Updates for Player {
    fn update(&mut self, keypress: Key, levelmap: &HashMap<Location, &Drawable>, universe: &Universe) -> ConflictHandler {
        let mut step = Location{ x: 0, y: 0 };
        match keypress {
            Key {code: Char, printable: 'j', ..} => {
                step.x = 0;
                step.y = 1;
            },
            Key {code: Char, printable: 'k', ..} => {
                step.x = 0;
                step.y = -1;
            },
            Key {code: Char, printable: 'l', ..} => {
                step.x = 1;
                step.y = 0;
            },
            Key {code: Char, printable: 'h', ..} => {
                step.x = -1;
                step.y = 0;
            },
            _ => {}
        }
        if universe.window_bounds.chk_inside(self.loc.chg(&step)) {
            self.loc = self.loc.chg(&step);
        }
        return ConflictHandler {kind: ConflictType::Nothing, loc: step, instigator: self,  parties: Vec::new()}
    }
}

struct Monster {
    loc: Location,
    sym: char,
    is_tame: bool,
    is_friendly: bool,
}

impl Drawable for Monster {
    fn draw(&self, con: &mut Root) {
        con.put_char(self.loc.x as i32, self.loc.y as i32, self.sym, BackgroundFlag::Set);
    }
}

impl Monster {
    // pub fn spawn_monster_rand(universe: &Universe, sym: char, is_tame: bool, is_friendly: bool) -> Rc<RefCell<Monster>> {
    pub fn spawn_monster_rand(universe: &Universe, sym: char, is_tame: bool, is_friendly: bool) -> Rc<Monster> {
        let x = random::<i32>() % universe.window_bounds.max.x ;
        let y = random::<i32>() % universe.window_bounds.max.y ;
        // Rc::new(RefCell::new(Monster { loc: Location { x : x, y : y }, sym : sym, is_tame : is_tame, is_friendly : is_friendly}))
        Rc::new(Monster { loc: Location { x : x, y : y }, sym : sym, is_tame : is_tame, is_friendly : is_friendly})
    }

    // pub fn spawn_monster_here(universe: &Universe, sym: char, is_tame: bool, is_friendly: bool, x: i32, y: i32) -> Rc<RefCell<Monster>> {
    pub fn spawn_monster_here(universe: &Universe, sym: char, is_tame: bool, is_friendly: bool, x: i32, y: i32) -> Rc<Monster> {
        // Rc::new(RefCell::new(Monster { loc: Location { x : x, y : y }, sym : sym, is_tame : is_tame, is_friendly : is_friendly}))
        Rc::new(Monster { loc: Location { x : x, y : y }, sym : sym, is_tame : is_tame, is_friendly : is_friendly})
    }
    // pub fn spawn_monster_near(universe: &Universe, sym: char, is_tame: bool, is_friendly: bool, loc_0: &Location, range: i32) -> Rc<RefCell<Monster>> {
    pub fn spawn_monster_near(universe: &Universe, sym: char, is_tame: bool, is_friendly: bool, loc_0: &Location, range: i32) -> Rc<Monster> {
        let mut loc = Location { x : 0, y : 0 };
        while loc.x == 0 && loc.y == 0 {
            loc.x = random::<i32>() % range;
            loc.y = random::<i32>() % range;
        }
        Rc::new(Monster { loc: loc.chg(loc_0), sym : sym, is_tame : is_tame, is_friendly : is_friendly})
    }
    fn random_move<'a>(&'a mut self, levelmap: &HashMap<Location, &'a Drawable>, universe: &Universe) -> ConflictHandler {
        // let mut step = self.loc ;
            // Location{ x: 0, y: 0};
        loop {
            let mut step = Location { x: 0, y: 0} ;
            match random::<i32>() % 4 {
            0 => {
                step.x = 0;
                step.y = 1;
            },
            1 => {
                step.x = 0;
                step.y = -1;
            },
            2 => {
                step.x = 1;
                step.y = 0;
            },
            3 => {
                step.x = -1;
                step.y = 0;
            },
                _ => {}
            }
            if universe.window_bounds.chk_inside(self.loc.chg(&step)) {
                match levelmap.get(&self.loc.chg(&step)) {
                    Some(occ) => {
                        return ConflictHandler {kind: ConflictType::Occupied, loc: self.loc.chg(&step), instigator: self,  parties: vec![occ.clone()]}
                    },
                    None => {
                        self.loc = self.loc.chg(&step);
                        return ConflictHandler {kind: ConflictType::Nothing, loc: self.loc.chg(&step), instigator: self,  parties: Vec::new()}
                    }
                }
            }
        }
    }
}

impl Updates for Monster {
    fn update<'a>(&'a mut self, keypress: Key, levelmap: &HashMap<Location, &'a Drawable>, universe: &Universe) -> ConflictHandler {
        if self.is_friendly {
            if self.is_tame {
                return self.random_move(levelmap, universe)
            } else {
                return self.random_move(levelmap, universe)
            }
        } else {
            return self.random_move(levelmap, universe)
        }
    }
}


fn render(player: &Player, level: &Level, universe: &Universe, con: &mut Root) {
    con.clear();
    player.draw(con);
    for monster in level.monsters.iter() {
        monster.draw(con);
    }
}

fn main() {
    // First, you must create the universe
    let mut universe = Universe {
        c_level: 0,
        exit: false,
        save: false,
        window_bounds: Bounds { min : Location { x : 0, y: 0 }, max : Location { x : 79,  y : 49 } },
        clock: 0,
    };
    // Then we can create the root window
    let mut root = Root::initializer()
        .size(universe.window_bounds.max.x+1 as i32, universe.window_bounds.max.y+1 as i32)
        .title("libtcot Rust tutorial")
        .fullscreen(false)
        .font("terminal.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .init();

    // Now lets make a bin for levels, as well as the first level
    let mut levels: Vec<Level> = Vec::new();
    levels.push(Level{ monsters: Vec::new(), map: HashMap::new() });

    // Lets initialize the player
    let mut player = Player {
        loc : Location { x : 40, y : 25 },
        sym : '@',
    };

    // Add the player's starting pet
    // levels[universe.c_level].add_monster(Monster::spawn_monster_near(&universe, 'd', true, false, &player.loc, 2));
    // Add a random monster
    // levels[universe.c_level].add_monster(Monster::spawn_monster_rand(&universe, 'r', true, false));

    // render(&player, &levels[universe.c_level], &universe, &mut root);

    root.flush();

    while !(root.window_closed() || universe.exit) {
        // Wait for and handle input
        let keypress = root.wait_for_keypress(true);
        match keypress {
            Key {code: Escape, ..} => universe.exit = true,
            _ => {
                let level = &mut levels[universe.c_level];
                player.update(keypress, &level.map, &universe);
                for monster in level.monsters.iter_mut() {
                    monster.update(keypress, &level.map, &universe);
                }
            }
        }
        render(&player, &levels[universe.c_level], &universe, &mut root);
        root.flush();
    }

}
