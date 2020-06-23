//! # Trident
//! a library for parsing triangular meshes
//!
//! This crate was developed so I can make 2d low-poly graphics with ggez.
//!
//! Here's an example `.tri` file:
//!
//! ```text
//! # This is a comment
//! 0, 0; 100, 0; 0, 70;. ff0000 # A red triangle
//! 100, 0; 100, 70, 0, 100;. 000000 # A black triangle
//! # This produces the anarchist flag.
//! ```
//!
//! 3 points in a line make a triangle. Having any more will make Trident fill in the shape with more triangles.
//!
//! ```text
//! 0, 30; 10, 0; 40, 0; 50, 30; 40, 60; 10, 60;. cccc00
//! # This produces a slightly wonky yellow hexagon.
//! ```
//!
//! # Grammar
//!
//! (This might be slightly inaccurate...)
//!
//! ```text
//! Shape ::= (Point ';')*3 (Point ';')* '.' Color ;
//! Point ::= Number ',' Number ;
//! Number ::= (Same as in Rust) ;
//! Color ::= Hex Hex Hex Hex Hex Hex (Hex Hex)? ;
//! Hex ::= ['0' '1' '2' '3' '4' '5' '6' '7' '8' '9' 'A' 'a' 'B' 'b' 'C' 'c' 'D' 'd' 'E' 'e' 'F' 'f'] ;
//! Comment ::= '#' Any* '\n' ;
//! ```
//!
//! Colors go `RRGGBBAA`. If `AA` is not supplied, it's assumed to be `ff` (aka opaque).

mod parse;

use nom::{error, multi::many0};

/// A Mesh is a bunch of Shapes.
#[derive(Debug)]
pub struct Mesh {
    pub shapes: Vec<Shape>,
}

impl Mesh {
    /// Parse the given string and create a Mesh from it.
    pub fn parse(input: &str) -> Result<Mesh, nom::Err<(&str, error::ErrorKind)>> {
        let (input, shapes) = many0(parse::line)(input)?;
        // It must consume everything
        if input.len() != 0 {
            return Err(nom::Err::Failure((input, error::ErrorKind::Eof)));
        }

        // Filter out all the None lines
        let filtered = shapes.into_iter().filter_map(|s| s).collect();

        Ok(Mesh { shapes: filtered })
    }
}

/// A Shape is one element. It can have any number of sides as long as it's more than 3.
/// Hopefully you know what a Shape is...
///
/// I'm not very good at writing documentation am I.
#[derive(Debug, PartialEq)]
pub struct Shape {
    /// All the points that constitute this Shape.
    pub points: Vec<Point>,
    /// The color of this Shape. Shapes can only have one color. (Add more Shapes if you want more colors in something.)
    pub color: (u8, u8, u8, u8),
}

/// A Point is a point in 2D space.
///
/// Points are `(x, y)`.
///
/// Positive X goes to the right. Positive Y goes down. (This is standard for computer graphics (but you probably knew that didn't you).)
pub type Point = (f32, f32);

/// A Color represents an RGBA color.
///
/// `(R, G, B, A)`
///
pub type Color = (u8, u8, u8, u8);

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
