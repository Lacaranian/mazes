mod gen;
mod maze_location;
mod maze_structure;
mod opt;
mod square_maze;

use structopt::StructOpt;

use crate::opt::MazeOpts;


fn main() {
	let opt = MazeOpts::from_args();

    println!("This doesn't do anything yet, but will hopefully soon generate mazes!");
}
