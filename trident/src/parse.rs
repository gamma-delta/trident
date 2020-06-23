//! This module parses strings into Meshes.

use super::*;

use nom::{
    bytes::complete::take,
    character::complete::{char, line_ending, not_line_ending, one_of},
    combinator::{map_res, opt},
    multi::{count, many0, many_till},
    number::complete::float,
    IResult,
};

/// Consumes 0 or more whitespace characters
fn whitespace(input: &str) -> IResult<&str, ()> {
    let (input, _) = many0(one_of(" \t"))(input)?;
    Ok((input, ()))
}

/// Parses a Point.
fn point(input: &str) -> IResult<&str, Point> {
    let (input, x) = float(input)?;
    // allow spaces between number & comma
    let (input, _) = whitespace(input)?;
    // the comma
    let (input, _) = char(',')(input)?;
    // more maybe whitespace
    let (input, _) = whitespace(input)?;
    // And the y
    let (input, y) = float(input)?;

    // Return OK!
    Ok((input, (x, y)))
}

/// Parses a Color.
fn color(input: &str) -> IResult<&str, Color> {
    let two_hexes = map_res(take(2usize), |s: &str| u8::from_str_radix(s, 16));

    let (input, r) = two_hexes(input)?;
    let (input, g) = two_hexes(input)?;
    let (input, b) = two_hexes(input)?;
    let (input, maybe_a) = opt(two_hexes)(input)?;
    let a = maybe_a.unwrap_or(0xff);

    Ok((input, (r, g, b, a)))
}

/// Parses a Shape
fn shape<'a>(input: &'a str) -> IResult<&str, Shape> {
    // Parses a single entry (Point;)
    let single_point = |input: &'a str| -> IResult<&'a str, Point> {
        let (input, _) = whitespace(input)?;
        let (input, p) = point(input)?;
        let (input, _) = whitespace(input)?;
        let (input, _) = char(';')(input)?;
        Ok((input, p))
    };

    let mut points = Vec::<Point>::new();

    // 3 points are required for a shape
    let (input, mut required_3_points) = count(single_point, 3)(input)?;
    points.append(&mut required_3_points);

    // But we might have extra points
    let (input, mut other_points) = many0(single_point)(input)?;
    points.append(&mut other_points);

    // Alright, now for the color?
    let (input, _) = whitespace(input)?;
    let (input, _) = char('.')(input)?;
    let (input, _) = whitespace(input)?;
    let (input, color) = color(input)?;

    // Lovely!
    Ok((input, Shape { color, points }))
}

/// Parses a Line and returns the Shape that might be in it.
/// (This strips away comments & such)
pub fn line(input: &str) -> IResult<&str, Option<Shape>> {
    let (input, _) = whitespace(input)?;
    let (input, maybe_comment) = opt(char('#'))(input)?;
    match maybe_comment {
        Some(_) => {
            // Comment line!
            let (input, _) = comment(input)?;
            Ok((input, None))
        }
        None => {
            // A shape line?
            let (input, maybe_shape) = opt(shape)(input)?;
            match maybe_shape {
                Some(shape) => {
                    // maybe whitespace...
                    let (input, _) = whitespace(input)?;
                    // and comment?
                    let (input, maybe_comment) = opt(char('#'))(input)?;
                    let (input, _) = match maybe_comment {
                        Some(_) => comment(input)?,
                        None => {
                            // Nope but gotta consume EOL
                            let (input, _) = opt(line_ending)(input)?;
                            (input, ())
                        }
                    };
                    Ok((input, Some(shape)))
                }
                None => {
                    // Blank line. consume the EOL and give None
                    let (input, _) = line_ending(input)?;
                    Ok((input, None))
                }
            }
        }
    }
}

/// Parses a comment.
/// You have to check for the '#' yourself.
/// This just throws away input till EOL/EOF
fn comment(input: &str) -> IResult<&str, ()> {
    // Throw away everything till EOL
    // (If there's no EOL that means it's EOF which is OK too)
    let (input, _) = opt(many_till(line_ending, not_line_ending))(input)?;
    // and consume the EOL too (or EOF)
    let (input, _) = opt(line_ending)(input)?;
    Ok((input, ()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_points() {
        let tests: Vec<(&str, Result<Point, ()>)> = vec![
            ("0, 0", Ok((0.0, 0.0))),
            ("4     ,       0.3", Ok((4.0, 0.3))),
            ("", Err(())),
        ];
        for (test, outcome) in tests {
            let raw_parsed = point(test);
            match raw_parsed {
                Ok((_, shape)) => assert_eq!(Ok(shape), outcome),
                Err(err) => {
                    if let Ok(_) = outcome {
                        panic!("{:?}", err)
                    }
                } // Else we wanted an error
            };
        }
    }

    #[test]
    fn parse_line() {
        let tests = vec![
            (
                "0,0; 10, 0; 10, 10;. ff0000",
                Some(Shape {
                    points: vec![(0.0, 0.0), (10.0, 0.0), (10.0, 10.0)],
                    color: (0xff, 0, 0, 0xff),
                }),
            ),
            (
                "0, 30; 10, 0; 40, 0; 50, 30; 40, 60; 10, 60;. cccc00 # This produces a slightly wonky yellow hexagon.",
                Some(Shape {
                    points: vec![(0.0, 30.0), (10.0, 0.0), (40.0, 0.0), (50.0, 30.0), (40.0, 60.0), (10.0, 60.0)],
                    color: (0xcc, 0xcc, 0, 0xff)
                })
            ),
            (
                "# An entirely commented line",
                None
            ),
            (
                "          0,0    ;     5.000000,      0.000   ; 0,                  12    ;    . abcdef33  ",
                Some(Shape {
                    points: vec![(0.0, 0.0), (5.0, 0.0), (0.0, 12.0)],
                    color: (0xab, 0xcd, 0xef, 0x33),
                })
            )
        ];
        for (test, outcome) in tests {
            let parsed = line(test);
            match parsed {
                Err(err) => panic!("{:?}", err),
                Ok((_, maybe_shape)) => assert_eq!(maybe_shape, outcome),
            }
        }
    }
}
