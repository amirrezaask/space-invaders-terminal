use ncurses::*;

const WELCOME: &str = "welcome to space invaders";
const PLAYER_SHIP: &'static str = "YOUR SHIP";
const ENEMY1_SHIP: &'static str = "@@";
const ROCKET: &'static str = "^";
const KEY_SPACE: i32 = ' ' as i32;
const KEY_EXIT: i32 = 'q' as i32;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
}

type ShipId = usize; // pointer to which ship in game struct

#[derive(Debug)]
struct Rocket {
    max_allowed_y: i32,
    max_allowed_x: i32,
    owner: ShipId, 
    pos: Position,
    shape: &'static str,
    direction: Direction,
    destroyed: bool,
}

impl Rocket {
    pub fn progress(&mut self) {
        match self.direction {
            Direction::Up => {
                self.pos.y -= 1;
                if self.pos.y < 0 {
                    self.destroyed = true;
                }
            }
            Direction::Down => {
                self.pos.y += 1;
                if self.pos.y > self.max_allowed_y {
                    self.destroyed = true;
                }

            }
        }
    }
    pub fn draw(&self) {
        if !self.destroyed {
            wmove(stdscr(), self.pos.y, self.pos.x);
            addstr(&self.shape);
        }
    }
    pub fn up(&mut self) {
        self.pos.up();
    }
    pub fn down(&mut self) {
        self.pos.down();
    }
    pub fn right(&mut self) {
        self.pos.right();
    }
    pub fn left(&mut self) {
        self.pos.left();
    }
}

#[derive(Debug, Clone)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    pub fn up(&mut self) {
        self.y -= 1;
    }
    pub fn down(&mut self) {
        self.y += 1;
    }
    pub fn right(&mut self) {
        self.x += 1;
    }
    pub fn left(&mut self) {
        self.x -= 1;
    }
}
#[derive(Debug, Clone)]
struct Ship {
    pos: Position,
    shape: &'static str,
    destroyed: bool,
}

impl Ship {
    pub fn draw(&self) {
        if !self.destroyed {
            wmove(stdscr(), self.pos.y, self.pos.x);
            addstr(&self.shape);
        }
    }

    pub fn up(&mut self) {
        self.pos.up();
    }
    pub fn down(&mut self) {
        self.pos.down();
    }
    pub fn right(&mut self) {
        self.pos.right();
    }
    pub fn left(&mut self) {
        self.pos.left();
    }

    
}
#[derive(Debug)]
struct Game {
    max_height: i32,
    max_width: i32,

    ships: Vec<Ship>,
    rockets: Vec<Rocket>,
    msg: Option<String>,
    done: bool,
}

fn create_enemy_grid(max_height: i32, max_width: i32, num_rows: i32, num_cols: i32) -> Vec<Ship> {
    let mut ships = Vec::<Ship>::new();
    let enemy_x_delta = 2;
    let x_offset =
        (max_width - ((enemy_x_delta * (num_cols - 1)) + (num_cols * ENEMY1_SHIP.len() as i32))) / 2;
    for i in 0..num_rows {
        for j in 0..num_cols {
            ships.push(Ship {
                pos: Position {
                    x: x_offset + (j * enemy_x_delta) + (j-1 * ENEMY1_SHIP.len() as i32),
                    y: (max_height / 8) + i as i32,
                },
                shape: ENEMY1_SHIP,
                destroyed: false,
            })
        }
    }

    ships
}

// fn ship_rocket_colision(rocket: &Rocket, ship: &Ship) -> bool {
// }

impl Game {
    pub fn new() -> Self {
        let mut max_height: i32 = 0;
        let mut max_width: i32 = 0;

        initscr();
        keypad(stdscr(), true);

        getmaxyx(stdscr(), &mut max_height, &mut max_width);
        let player = Ship {
            pos: Position {
                x: (max_width - PLAYER_SHIP.len() as i32) / 2,
                y: (max_height - max_height / 6),
            },
            shape: PLAYER_SHIP,
            destroyed: false,
        };

        let mut ships: Vec<Ship> = create_enemy_grid(max_height, max_width, 5, 12);
        ships.insert(0, player);
        Self {
            max_height,
            max_width,
            ships,
            rockets: vec![],
            msg: None,
            done: false,
        }
    }
    pub fn clear(&self) {
        clear();
    }
    pub fn shoot_rocket(&mut self, idx: ShipId) {
        let ship = &self.ships[idx];
        self.rockets.push(Rocket {
            max_allowed_x: self.max_height,
            max_allowed_y: self.max_width,
            pos: Position { x: ship.pos.x + ship.shape.len() as i32/2, y: ship.pos.y-1 },
            shape: ROCKET,
            direction: Direction::Up,
            owner: idx,
            destroyed: false,
                
        });
    }
    fn update_states(&mut self) {
        // update rockets
        for rocket in self.rockets.iter_mut() {
            rocket.progress();
        }
        // find colisions of rockets and ships
        for ship in self.ships.iter_mut () {
            for rocket in self.rockets.iter_mut() {
                if !rocket.destroyed && !ship.destroyed && ship.pos.x == rocket.pos.x && ship.pos.y == rocket.pos.y  {
                    ship.destroyed = true;
                    rocket.destroyed = true;
                }
            }
        }
        // // remove destroyed rockets
        // for (idx, _) in self.rockets.iter_mut().enumerate() {
        //     if self.rockets[idx].destroyed {
        //         self.rockets.remove(idx);
        //     }

        // }

        // // ships
        // if self.ships.len() > 1 {
        //     for idx in 0..self.ships.len() -1 {
        //         if self.ships[idx].destroyed {
        //             self.ships.remove(idx);
        //         }

        //     }
        // }
        
    }
    fn render(&mut self) {
        self.update_states();
        self.clear();
        if self.msg.is_some() {
            self.print_center(self.msg.clone().unwrap().as_str());
            self.msg = None;
            getch();
            self.clear();
        }
        //draw enemies
        for ship in self.ships.iter_mut() {
            ship.draw();
        }

        //draw rockets
        for rocket in self.rockets.iter_mut() {
            rocket.draw();
        }

   }

    pub fn print_center(&self, text: &str) {
        let center = (
            (self.max_height / 2),
            ((self.max_width - text.len() as i32) / 2),
        );
        wmove(stdscr(), center.0, center.1);
        addstr(text);
    }

    pub fn start(&mut self) {
        self.msg = Some(WELCOME.to_string());
        self.render();
        getch();
        halfdelay(1);
        loop {
            if self.done {
                break;
            }
            self.render();

            let player_move = getch();
            match player_move {
                KEY_LEFT => {
                    self.ships[0].left();
                    self.update_states();
                }
                KEY_RIGHT => {
                    self.ships[0].right();
                    self.update_states();
                }
                KEY_UP => {
                    self.ships[0].up();
                    self.update_states();
                }
                KEY_DOWN => {
                    self.ships[0].down();
                    self.update_states();
                }
                KEY_SPACE => {
                    self.shoot_rocket(0);
                    self.update_states();
                }
                KEY_EXIT => break,
                ERR => { // user did not enter any thing just update states
                   continue;
                }
                _ => {
                    self.msg = Some("unkown key".to_string());
                }
            }
        }
    }
}

fn main() {
    let mut game = Game::new();
    game.start();
    println!("\n\n{:?}", game);
}
