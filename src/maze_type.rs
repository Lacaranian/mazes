use std::fmt;
use std::str;

#[derive(Debug, PartialEq)]
pub enum MazeType {
	Square
}

impl Default for MazeType {
	fn default() -> Self { MazeType::Square }
}

impl fmt::Display for MazeType {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl str::FromStr for MazeType {
	type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed = match s.to_ascii_lowercase().as_str() {
        	"square" => MazeType::Square,
        	_ => Err(String::from("Could not parse MazeType as string!"))?
        };

        Ok(parsed)
    }
}