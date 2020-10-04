use structopt::StructOpt;

use crate::maze_location::MazeLocation;
use crate::maze_structure::MazeStructure;

#[derive(StructOpt, Debug)]
#[structopt(name = "mazes")]
/// Generate or solve a maze!
pub struct MazeOpts {
	#[structopt(short, long, default_value)]
	/// The type of maze to generate or solve
	structure: MazeStructure,
	#[structopt(short, long, default_value)]
	/// Where the maze solver is intended to begin
	origin: MazeLocation,
	#[structopt(short, long, default_value)]
	/// Where the maze solver is intended to end
	goal: MazeLocation,
	#[structopt(short, long, default_value = "100")]
	/// The number of "nodes" in the maze's graph
	/// Each type of maze may interpret this differently
	pub size: usize,
}