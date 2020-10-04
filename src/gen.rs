use oorandom::Rand64;
use chrono::prelude::Utc;

use crate::opt::MazeOpts;
use crate::square_maze::SquareMaze;

// Steps of Maze generation v1
// 1. Start with generating an abstract space based on the size of the maze
// 2. Use the origin/end constraints to fill in some initial parts of the maze
// 3. Randomly generate parts of the maze, obeying other constraints of the maze
// 4. When nothing is left to generate, the maze exists in an abstract form 
//    Map the graph for the maze to an image file
// 5. Optionally include a companion image that provides the solution for the maze (using a solver on the abstract structure)

pub fn generate_maze(opts: &MazeOpts) -> () {
	let now = Utc::now().timestamp_millis() as u128;
	let prng = Rand64::new(now);

	// For now, ignore the abstract concept of a maze and most options, get a concrete example first
	let side = (opts.size as f64).sqrt().ceil() as usize;
	let fresh_maze = SquareMaze::start_maze(side, side);

	// Handle the origin
	// Handle the destination
	todo!();
}

