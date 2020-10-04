use std::fmt;
use std::str;

#[derive(Debug, PartialEq)]
/// A location in or around the maze, used as a constraint in generation
pub enum MazeLocation {
    /// The classic location of the minotaur, often the goal of Labyrinths
	Center,
    /// An arbitrary outer surface of the maze, 
    OuterEdge
}

impl Default for MazeLocation {
	fn default() -> Self { MazeLocation::Center }
}

impl fmt::Display for MazeLocation {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl str::FromStr for MazeLocation {
	type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed = match s.to_ascii_lowercase().as_str() {
        	"center" => MazeLocation::Center,
            "edge" | "outeredge" => MazeLocation::OuterEdge,
        	_ => Err(String::from("Could not parse MazeLocation as string!"))?
        };

        Ok(parsed)
    }
}