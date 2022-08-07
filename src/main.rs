use ncurses::*;

const WELCOME: &str = "welcome to space invaders";
const PLAYER_SHIP: &str = "&&&&";

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

struct Game {
    max_height: i32,
    max_width: i32,

    player_pos: Position,
    msg: Option<String>,
    done: bool
}

impl Game {
    pub fn new() -> Self {
        let mut max_height: i32 = 0;
        let mut max_width: i32 = 0;

        initscr();
        keypad(stdscr(), true);

        getmaxyx(stdscr(), &mut max_height, &mut max_width);

        Self {
            max_height,
            max_width,
            player_pos: Position {
                x: (max_width - PLAYER_SHIP.len() as i32) / 2,
                y: (max_height - max_height / 6),
            },
            msg: None,
            done: false,
        }
    }
    pub fn clear(&self) {
        clear();
    }

    fn render(&mut self) {
        self.clear();
        if self.msg.is_some() {
            self.print_center(self.msg.clone().unwrap().as_str());
            self.msg = None;
            return;
        }
        // draw player
        wmove(stdscr(), self.player_pos.y, self.player_pos.x);
        addstr(PLAYER_SHIP);
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
                    self.player_pos.left();
                }
                KEY_RIGHT => {
                    self.player_pos.right();
                }
                KEY_UP => {
                    self.player_pos.up();
                }
                KEY_DOWN => {
                    self.player_pos.down();
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
