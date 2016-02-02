extern crate tcod;
extern crate rand;
use std::cmp::{PartialEq};
use tcod::{BackgroundFlag};
use tcod::console::{Console, Root, FontLayout, FontType};
use tcod::input::{Key};
use tcod::input::KeyCode::{Escape, Char};
use rand::random;

trait Drawable {
    fn draw(&self, &mut Root);
}

trait Updates {
    fn update(&mut self, Key, &mut Level, &Universe);
}

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

struct Level {
    // num: i32,
    monsters: Vec<Box<Monster>>,
}

struct Location {
    x: i32,
    y: i32,
}

impl Location {
    fn chg(&self, chg: &Location) -> Location {
        Location { x : self.x + chg.x, y : self.y + chg.y }
    }
}

impl PartialEq for Location {
    fn eq(&self, other: &Location) -> bool {
        self.x == other.x && self.y == other.y
    }
}

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
    fn update(&mut self, keypress: Key, level: &mut Level, universe: &Universe) {
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
    }
}

struct Monster {
    loc: Location,
    sym: char,
    is_tame: bool,
    is_friendly: bool,
}

impl Drawable for Monster {
    fn draw(&self, con: &mut Root) -> () {
        con.put_char(self.loc.x as i32, self.loc.y as i32, self.sym, BackgroundFlag::Set);
    }
}

impl Monster {
    fn random_move(&mut self, level: &mut Level, universe: &Universe) {
        let mut step = Location{ x: 0, y: 0};
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
            self.loc = self.loc.chg(&step);
        }
    }
}

impl Updates for Monster {
    fn update(&mut self, keypress: Key, level: &mut Level, universe: &Universe) {
        if self.is_friendly {
            if self.is_tame {
                self.random_move(level, universe);
            } else {
                self.random_move(level, universe);
            }
        } else {
            self.random_move(level, universe);
        }
    }
}

fn spawn_monster_rand(universe: &Universe, sym: char, is_tame: bool, is_friendly: bool) -> Box<Monster> {
    let x = random::<i32>() % universe.window_bounds.max.x ;
    let y = random::<i32>() % universe.window_bounds.max.y ;
    Box::new(Monster { loc: Location { x : x, y : y }, sym : sym, is_tame : is_tame, is_friendly : is_friendly})
}

fn spawn_monster_here(universe: &Universe, sym: char, is_tame: bool, is_friendly: bool, x: i32, y: i32) -> Box<Monster> {
    Box::new(Monster { loc: Location { x : x, y : y }, sym : sym, is_tame : is_tame, is_friendly : is_friendly})
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
        c_level: 1,
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
    levels.push(Level{ monsters: Vec::new() });

    // Lets initialize the player
    let mut player = Player {
        loc : Location { x : 40, y : 25 },
        sym : '@',
    };

    // Lets add the player's pet
    levels[universe.c_level].monsters.push(Box::new(Monster {
        loc: Location { x: 40, y: 25 },
        sym: 'd',
        is_tame: true,
        is_friendly: true,
    }));
    levels[universe.c_level].monsters[0].random_move(&mut levels[universe.c_level], &universe);

    // Add a random monster
    levels[universe.c_level].monsters.push(spawn_monster_rand(&universe, 'r', true, false));

    render(&player, &levels[universe.c_level], &universe, &mut root);

    root.flush();

    while !(root.window_closed() || universe.exit) {
        // Wait for and handle input
        let keypress = root.wait_for_keypress(true);
        match keypress {
            Key {code: Escape, ..} => universe.exit = true,
            _ => {
                player.update(keypress, &mut levels[universe.c_level], &universe);
                for monster in levels[universe.c_level].monsters.iter_mut() {
                    monster.update(keypress, &mut levels[universe.c_level], &universe)
                }
            }
        }
        render(&player, &levels[universe.c_level], &universe, &mut root);
        root.flush();
        // io::stdout().flush().unwrap();
    }

}
