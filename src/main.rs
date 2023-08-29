use clap::{Arg, Command};
use life::{clrscr, game::grid::*};

fn main() {
    let command = Command::new("Conway's Game of Life")
        .version("0.1.0")
        .author("Evgeny K.")
        .about("Conway's Game of Life implementation in rust")
        .long_about(None)
        .arg(
            Arg::new("width")
                .short('W')
                .long("width")
                .help("Width of the grid")
                .default_value("25")
                .required(false),
        )
        .arg(
            Arg::new("height")
                .short('H')
                .long("height")
                .help("Height of the grid")
                .default_value("25")
                .required(false),
        )
        .arg(
            Arg::new("fps")
                .short('F')
                .long("fps")
                .help("Frames per second")
                .default_value("20")
                .required(false),
        )
        .get_matches();
    let width: u32 = command.get_one::<String>("width").unwrap().parse().unwrap();
    let height: u32 = command
        .get_one::<String>("height")
        .unwrap()
        .parse()
        .unwrap();
    let fps: u32 = command.get_one::<String>("fps").unwrap().parse().unwrap();
    let mut random_grid = Grid::new(width, height, Some(true));
    // sleep
    std::thread::sleep(std::time::Duration::from_millis(50));
    while random_grid.next_generation() {
        clrscr();
        println!("{}", random_grid);
        std::thread::sleep(std::time::Duration::from_millis((1000 / fps).into()));
    }
}
