use ncurses;

const WELCOME: &str = "welcome to space invaders.\n";

fn main() {
    ncurses::initscr();
    let mut HEIGHT: i32 = 0; //y
    let mut WIDTH: i32 = 0; // x

    ncurses::getmaxyx(ncurses::stdscr(), &mut HEIGHT, &mut WIDTH);

    
    ncurses::wmove(ncurses::stdscr(), HEIGHT/2, (WIDTH- WELCOME.len() as i32) / 2);
    ncurses::addstr(WELCOME);

    ncurses::getch();
    
}
