mod maze_type;
mod opt;

use structopt::StructOpt;

use crate::opt::MazesOpts;


fn main() {
	let opt = MazesOpts::from_args();
    println!("This doesn't do anything yet, but will hopefully soon generate mazes!");
}
