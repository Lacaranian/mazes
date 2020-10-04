use structopt::StructOpt;

use crate::maze_type::MazeType;

#[derive(StructOpt, Debug)]
#[structopt(name = "mazes")]
/// Generate or solve a maze!
pub struct MazesOpts {
	#[structopt(short, long, default_value)]
	/// The type of maze to generate or solve
	maze: MazeType,
}