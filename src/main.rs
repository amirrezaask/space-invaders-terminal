use ncurses::*;
use rand::distributions::{Distribution, Standard};
use rand::Rng;

const WELCOME: &str = "welcome to space invaders";
const PLAYER_SHIP: &'static str = "YOUR SHIP";
const YOU_WON: &'static str = "YOU WON, NICE JOB";
const YOU_LOST: &'static str = "GET GOOOD PLS";
const ENEMY1_SHIP: &'static str = "@@";
const ROCKET: &'static str = "^";
const KEY_SPACE: i32 = ' ' as i32;
const KEY_EXIT: i32 = 'q' as i32;
const ENEMY_GRID: (i32, i32) = (5, 5); 

#[derive(Debug, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl Direction {
    pub fn random() -> Direction {
        let mut rng = rand::thread_rng();
        let number = rng.gen_range(1..=8);

        return match number {
            1 => Direction::Up,
            2 => Direction::Down,
            3 => Direction::Right,
            4 => Direction::Left,
            5 => Direction::UpLeft,
            6 => Direction::UpRight,
            7 => Direction::DownLeft,
            8 => Direction::DownRight,
            _ => unreachable!()
        }
    }
}




type ShipId = usize; // pointer to which ship in game struct

#[derive(Debug, Clone)]
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
            _ => {}
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
    pub fn move_into(&mut self, d: &Direction) {
        match d {
            Direction::Up => {
                self.up();
            }
            Direction::Down => {
                self.down();
            }

            Direction::Left => {
                self.left();
            }
            Direction::Right => {
                self.right();
            }
            Direction::UpLeft => {
                self.up();
                self.left();
            }
            Direction::UpRight => {
                self.up();
                self.right();
            }
            Direction::DownLeft => {
                self.down();
                self.left();
            }
            Direction::DownRight => {
                self.down();
                self.right();
            }

        }
    }
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

#[derive(Debug, Clone, PartialEq)]
enum Side {
    Player,
    Enemy,
}
#[derive(Debug, Clone)]
struct Ship {
    pos: Position,
    shape: &'static str,
    destroyed: bool,
    side: Side,
}

impl Ship {
    pub fn hit_boxes(&self) -> Vec<(i32, i32)> {
        let mut hits = Vec::<(i32, i32)>::new();
        for x in self.pos.x..self.pos.x+self.shape.len() as i32 {
            hits.push((x, self.pos.y));
        }

        return hits;
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
#[derive(Debug, Clone, PartialEq, Eq)]
enum GameResult {
    Unknown,
    Won,
    Lost,
}
#[derive(Debug)]
struct Game {
    max_height: i32,
    max_width: i32,

    ships: Vec<Ship>,
    rockets: Vec<Rocket>,
    msg: Option<String>,
    result: GameResult,
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
                side: Side::Enemy,
            })
        }
    }

    ships
}

fn ship_rocket_colision(rocket: &Rocket, ship: &Ship) -> bool {
    if !rocket.destroyed && !ship.destroyed {
        return ship.hit_boxes().contains(&(rocket.pos.x, rocket.pos.y));
    } else {
        return false;
    }
}

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
            side: Side::Player,
        };

        let mut ships: Vec<Ship> = create_enemy_grid(max_height, max_width, ENEMY_GRID.0, ENEMY_GRID.1);
        ships.insert(0, player);
        Self {
            max_height,
            max_width,
            ships,
            rockets: vec![],
            msg: None,
            result: GameResult::Unknown,
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
    fn valid_move_for_a_ship(max_height:i32, max_width: i32, ship: &Ship, d: &Direction) -> bool {
        match d {
            Direction::Up => {
                if ship.pos.y-1 < 1 {
                    return false;
                }
            }

            Direction::Down => {
                if ship.pos.y+1 > max_height-1 {
                    return false;
                }
            }

            Direction::Left => {
                if ship.pos.x-1 < 1 {
                    return false;
                }
            }

            Direction::Right => {
                if ship.pos.x+1 > max_width-1 {
                    return false;
                }
            }

            Direction::UpLeft => {
                if ship.pos.y-1 < 1 || ship.pos.x-1 < 1 {
                    return false;
                }
            }
            Direction::UpRight => {
                if ship.pos.y-1 < 1 || ship.pos.x+1 > max_width - 1 {
                    return false;
                }
            }
            Direction::DownLeft => {
                if ship.pos.y+1 > max_height-1 || ship.pos.x-1 < 1 {
                    return false;
                }

            }
            Direction::DownRight => {
                if ship.pos.y+1 > max_height-1 || ship.pos.x+1 > max_width -1 {
                    return false;
                }

            }

        }


        return true;
    }
    fn update_states(&mut self) {
        // update rockets
        for rocket in self.rockets.iter_mut() {
            rocket.progress();
        }

        // move enemy ships
        for idx in 1..self.ships.len() {
            let mut ship = &mut self.ships[idx];
            loop {
                let direction = Direction::random();
                if Self::valid_move_for_a_ship(self.max_height, self.max_width, ship, &direction) {
                    ship.pos.move_into(&direction);
                    break;
                }
            }
            
        }
        // find colisions of rockets and ships
        for ship in self.ships.iter_mut(){
            for rocket in self.rockets.iter_mut() {
                if ship_rocket_colision(rocket, ship)  {
                    ship.destroyed = true;
                    rocket.destroyed = true;
                }
            }
        }
        // remove destroyed rockets and ships
        self.rockets = self.rockets.clone().into_iter().filter(|rocket| !rocket.destroyed).collect();
        self.ships = self.ships.clone().into_iter().filter(|ship| !ship.destroyed).collect();
        
        if self.ships.len() < 2 {
            match self.ships.len() {
                1 => {
                    if self.ships[0].side == Side::Player {
                        self.result = GameResult::Won;
                    } else {
                        self.result = GameResult::Lost;
                    }
                }
                0 => {
                    self.result = GameResult::Lost;
                }
                _ => {
                    panic!()
                }
            }
        }
        
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

        if self.result != GameResult::Unknown {
            match self.result {
                GameResult::Won => {
                    self.print_center(YOU_WON);
                    self.done = true;
                    nocbreak();
                    getch();
                }
                GameResult::Lost => {
                    self.print_center(YOU_LOST);
                    self.done = true;
                    nocbreak();
                    getch();
                }

                _ => {

                }
            }

            return;

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
}
