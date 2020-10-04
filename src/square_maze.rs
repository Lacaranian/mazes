use oorandom::Rand64;

use crate::maze_location::MazeLocation;

/// A maze with a squre structure to its spaces.
///
/// By caring only about the "edges" of the square maze, we can specify the whole maze by tracking 
/// which edges are walls and which are paths. The ol negative space switcheroo', one might say.
///
/// For example, this set of four spaces with four walls each (4 of which are shared):
///  _ _
/// |_|_|
/// |_|_|
///
/// is equivalent to a set of 12 edges, all of which have 2 adjacent spaces.
///
/// A square maze can be considered complete when all edges are specified (not Unknown), but it is only 
/// finishable if certain other constraints are met.
pub struct SquareMaze {
	/// X/Y dimensions - track these mainly for efficient lookups
	/// The "square" here refers to the structure of individual spaces in the maze, so the maze itself 
	/// may in fact be rectangular.
	dimensions: Point,
	/// By convention, (0,0) is at the top-left, row-major (each Vec<Edge> is a row of edges)
	/// The number of edges will alternate each row, so this is _not_ uniform
	/// Horizontal edges correspond to even indices, and vertical edges correspond to odd indices, of the overall Vec<Vec<Edge>>
	edges: Vec<Vec<Edge>>
}

type Point = (u64, u64);

impl SquareMaze {
	// Create a box, with Wall exterior edges and Unknown interior edges
	// Requires a positive integer width or height
	pub fn start_maze(width: usize, height: usize) -> Self {
		let num_edge_rows = 2 * height + 1;
		let mut edge_rows = Vec::with_capacity(num_edge_rows);

		let first_row = vec![Edge::Wall; width];
		edge_rows.push(first_row);

		for idx in 1..(num_edge_rows - 1) {
			if idx % 2 == 1 {
				let mut mid_row = vec![Edge::Wall; 1];
				mid_row.append(&mut vec![Edge::Unknown; width - 1]);
				mid_row.push(Edge::Wall);

				edge_rows.push(mid_row);
			} else {
				edge_rows.push(vec![Edge::Unknown; width]);
			}
		}

		let last_row = vec![Edge::Wall; width];
		edge_rows.push(last_row);


		SquareMaze {
			dimensions: (width as u64, height as u64),
			edges: edge_rows
		}
	}

	fn with_origin(&mut self, prng: &mut Rand64, _origin: MazeLocation) -> &mut Self {
		// Assume OuterEdge is being used for this initial version
		// Poke a hole in a random outer wall of the maze
		let (width, height) = self.dimensions;
		let number_outer_edges = 2 * width + 2 * height;
		let chosen_edge = prng.rand_range(0..number_outer_edges);
		
		todo!();
	}

	// Indexed clockwise from the top edge
	fn outer_edge_coord(&self, chosen_edge: u64) -> Point {
		let (width, height) = self.dimensions;
		let first_offset = chosen_edge - width;
		
		if first_offset < 0 {
			(chosen_edge, 0)
		} else {
			let second_offset = first_offset - height;
			if second_offset < 0 {
				(width, 2 * first_offset + 1)
			} else {
				let third_offset = second_offset - width;
				if third_offset < 0 {
					(width - second_offset, height)
				} else {
					(0, 2 * height - third_offset - 1)
				}	
			}
		}
	}

	// NESW order
	fn get_edges_of_space(&self, x: u64, y: u64) -> (Point, Point, Point, Point) {
		let top = (x, y);
		let right = (x + 1 , y + 1);
		let bottom = (x, y + 2);
		let left = (x , y + 1);

		(top, right, bottom, left)
	}

	fn get_spaces_of_edge(&self, x: u64, y: u64) -> (Option<Point>, Option<Point>) {
		let (width, height) = self.dimensions;

		if y % 2 == 0 { // Horizontal edge
			let top = Some(y / 2 - 1)
				.filter(|space_y| 0 <= *space_y)
				.map(|space_y| (x, space_y));
			let bottom = Some(y / 2)
				.filter(|space_y| *space_y < height)
				.map(|space_y| (x, space_y));
			(top, bottom)
		} else { // Vertical edge
			let left = Some(x - 1)
				.filter(|space_x| 0 <= *space_x)
				.map(|space_x| (space_x, y - 1));
			let right = Some(x)
				.filter(|space_x| *space_x < width)
				.map(|space_x| (space_x, y - 1));
			(left, right)
		}
	}
}

#[derive(Clone)]
pub enum Edge {
	Unknown,
	Wall,
	Path
}

