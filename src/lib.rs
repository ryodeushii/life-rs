pub mod args;
pub mod game;
pub fn clrscr() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}
