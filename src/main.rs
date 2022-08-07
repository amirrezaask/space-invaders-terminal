use ncurses::*;

const WELCOME: &str = "welcome to space invaders";
const PLAYER_SHIP: &'static str = "YOUR SHIP";
const ENEMY1_SHIP: &'static str = "@@";
const ROCKET: &'static str = "^";
const KEY_SPACE: i32 = ' ' as i32;
const KEY_EXIT: i32 = 'q' as i32;

#[derive(Debug)]
struct Rocket {
    pos: Position,
    shape: &'static str,
}

impl Rocket {
    pub fn draw(&self) {
        wmove(stdscr(), self.pos.y, self.pos.x);
        addstr(&self.shape);
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
}

impl Ship {
    pub fn draw(&self) {
        wmove(stdscr(), self.pos.y, self.pos.x);
        addstr(&self.shape);
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

    player: Ship,
    enemies: Vec<Ship>,
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
            })
        }
    }

    ships
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
        };

        let enemies: Vec<Ship> = create_enemy_grid(max_height, max_width, 5, 12);
        Self {
            max_height,
            max_width,
            player,
            enemies,
            rockets: vec![],
            msg: None,
            done: false,
        }
    }
    pub fn clear(&self) {
        clear();
    }
    pub fn ship_shoots(&mut self, ship: &Ship) {
        self.rockets.push(Rocket {
            pos: Position { x: ship.pos.x + ship.shape.len() as i32/2, y: ship.pos.y-1 },
            shape: ROCKET,
        });
    }
    fn render(&mut self) {
        self.clear();
        if self.msg.is_some() {
            self.print_center(self.msg.clone().unwrap().as_str());
            self.msg = None;
            getch();
            self.clear();
        }
        // draw player
        self.player.draw();

        //draw enemies
        for enemy in self.enemies.iter_mut() {
            enemy.draw();
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
        loop {
            if self.done {
                break;
            }
            self.render();

            let player_move = getch();

            match player_move {
                KEY_LEFT => {
                    self.player.left();
                }
                KEY_RIGHT => {
                    self.player.right();
                }
                KEY_UP => {
                    self.player.up();
                }
                KEY_DOWN => {
                    self.player.down();
                }
                KEY_SPACE => {
                    self.ship_shoots(&self.player.clone());
                }
                KEY_EXIT => break,
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
