use std::fmt;
use std::str;

#[derive(Debug, PartialEq)]
/// The spatial strucure of the maze
pub enum MazeStructure {
    /// Each space is a square. Each edge be blocked by a wall, or open to the adjacent space
	Square,
	/// A set of cocentric rings, with walls dividing the rings from each other, as well
	/// as arcs within a ring.
	Rings
}


impl Default for MazeStructure {
	fn default() -> Self { MazeStructure::Square }
}

impl fmt::Display for MazeStructure {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl str::FromStr for MazeStructure {
	type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed = match s.to_ascii_lowercase().as_str() {
        	"square" => MazeStructure::Square,
        	_ => Err(String::from("Could not parse MazeStructure as string!"))?
        };

        Ok(parsed)
    }
}