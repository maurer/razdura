extern crate tcod;
extern crate rand;
use std::cmp::{PartialEq};
use tcod::{BackgroundFlag};
use tcod::console::{Console, Root, FontLayout, FontType};
use tcod::input::{Key};
use tcod::input::KeyCode::{Escape, Char};
use rand::random;

trait Drawable {
    fn draw<T:Console>(&self, &mut T) -> ();
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


struct Creature {
    loc: Location,
    sym: char,
}

impl Drawable for Creature {
    fn draw<T: Console>(&self, con: &mut T) -> () {
        con.put_char(self.loc.x, self.loc.y, self.sym, BackgroundFlag::Set);
    }
}


fn render<T: Console>(con: &mut T, player: &Creature, pets: &Vec<&mut Creature>, monsters: &Vec<&mut Creature>) {
    con.clear();
    player.draw(con);
    for pet in pets {
        pet.draw(con);
    }
    for monster in monsters{
        monster.draw(con);
    }
}

fn main() {
    let window_bounds = Bounds { min : Location { x : 0, y: 0 }, max : Location { x : 79,  y : 49 } };
    let mut root = Root::initializer()
        .size(window_bounds.max.x+1, window_bounds.max.y+1)
        .title("libtcot Rust tutorial")
        .fullscreen(false)
        .font("terminal.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .init();
    let mut exit = false;

    let mut clock: i32 = 0;
    let mut step: Location = Location { x : 0, y : 0 };

    let mut player = Creature { loc : Location { x: 40, y: 25 }, sym: '@' };
    let mut pet0 = Creature { loc : Location {x: 40, y: 25 }, sym: 'd' };

    let mut pets: Vec<&mut Creature> = Vec::new();
    let mut monsters: Vec<&mut Creature> = Vec::new();

    pets.push(&mut pet0);

    render(&mut root, &player, &pets, &monsters);

    root.flush();

    while !(root.window_closed() || exit) {
        // Wait for and handle input
        let keypress = root.wait_for_keypress(true);
        match keypress {
            Key {code: Escape, ..} => exit = true,
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
        if window_bounds.chk_inside(player.loc.chg(&step)) {
            player.loc = player.loc.chg(&step);
        }
        for pet in pets.iter_mut() {
            match random::<u8>() % 4 {
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
            if window_bounds.chk_inside(pet.loc.chg(&step)) && (pet.loc.chg(&step) != player.loc) {
                pet.loc = pet.loc.chg(&step);
            }
        }
        // render(&mut root, & live);
        // render(&mut root, & [& player, &pet0]);
        render(&mut root, &player, &pets, &monsters);
        root.flush();
        clock += 1;
        // io::stdout().flush().unwrap();
    }

}
