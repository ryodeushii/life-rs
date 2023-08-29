use clap::{arg, Arg, Command};
use life::game::grid::*;

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
        .get_matches();
    let width: u32 = command.get_one::<String>("width").unwrap().parse().unwrap();
    let height = command
        .get_one::<String>("height")
        .unwrap()
        .parse()
        .unwrap();
    let mut random_grid = Grid::new(width, height, Some(true));
    // sleep
    std::thread::sleep(std::time::Duration::from_millis(50));
    while random_grid.next_generation() {
        println!("{}", random_grid);
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
    println!("Final generation: {}", random_grid);
    println!(
        "It took {} generations to stabilize",
        random_grid.generation
    );
}
