use super::{network::NodeId, primitives::Point};
use geometry_predicates::orient2d;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Copy, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "snake_case")]
/// A structure holding a microfluidic channel
pub struct Channel {
    /// Id of the channel
    pub id: usize,

    /// Start node
    pub node_a: NodeId,

    /// End node
    pub node_b: NodeId,

    /// Channel Shape
    pub shape: Shape,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "snake_case")]
/// Channel shape/cross-section types
pub enum Shape {
    /// Rectangular channel cross-section variant
    Rectangular(RectangularShape),

    /// Circular channel cross-section variant
    Cylindrical(CylindricalShape),
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "snake_case")]
/// Rectangular channel cross-section
pub struct RectangularShape {
    /// Channel width
    pub width: f64,

    /// Channel height
    pub height: f64,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "snake_case")]
/// Round channel cross-section
pub struct CylindricalShape {
    /// Cross-section radius
    pub radius: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "snake_case")]
/// A continuous channel (Dubin's) path with arcs and straight segments
pub struct ChannelPath {
    /// Single pieces of the path
    pub pieces: Vec<PathPiece>,
}

impl ChannelPath {
    pub fn new() -> Self {
        ChannelPath { pieces: Vec::new() }
    }

    pub fn add(&mut self, piece: PathPiece) {
        self.pieces.push(piece)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct PathLength(pub f64);

pub trait SVGPath {
    fn svg_path_command(&self, invert_y: bool) -> String;
    fn length(&self) -> PathLength;
}

impl SVGPath for ChannelPath {
    fn svg_path_command(&self, invert_y: bool) -> String {
        if self.pieces.len() == 0 {
            return "".to_string();
        }

        let Point([x, y]) = match &self.pieces[0] {
            PathPiece::Arc(arc) => arc.start,
            PathPiece::LineSegment(line) => line.start,
        };

        let mut s = format!("M {x} {y} ").to_owned();
        for piece in self.pieces.iter() {
            match piece {
                PathPiece::Arc(arc) => s.push_str(&arc.svg_path_command(invert_y)),
                PathPiece::LineSegment(line) => s.push_str(&line.svg_path_command(invert_y)),
            }
        }
        s.to_string()
    }

    fn length(&self) -> PathLength {
        PathLength(
            self.pieces
                .iter()
                .map(|p| match p {
                    PathPiece::Arc(arc) => arc.length().0,
                    PathPiece::LineSegment(ls) => ls.length().0,
                })
                .sum(),
        )
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "snake_case")]
/// Single piece of a path, can be of several types
pub enum PathPiece {
    /// Circular arc segment
    Arc(Arc),

    /// Straight line segment
    LineSegment(LineSegment),
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "snake_case")]
/// A straight line segment
pub struct LineSegment {
    /// starting point of the segment
    pub start: Point,

    /// end point of the segment
    pub end: Point,
}

impl SVGPath for LineSegment {
    fn svg_path_command(&self, _: bool) -> String {
        let Point([x, y]) = self.end;
        format!("L {x} {y} ")
    }

    fn length(&self) -> PathLength {
        let Point([sx, sy]) = self.start;
        let Point([ex, ey]) = self.end;
        PathLength(f64::hypot(sx - ex, sy - ey))
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "snake_case")]
/// Arc segment
pub struct Arc {
    /// Clockwise rotation of the arc from start through end; right=true means clockwise, counterclockwise otherwise. Mathematical Y axis assumed.
    pub right: bool,

    /// Starting point of the segment
    pub start: Point,

    /// End point of the segment
    pub end: Point,

    /// Center point of the arc's complete circle
    pub center: Point,
}

#[derive(Debug, PartialEq)]
struct Radius(f64);

#[derive(Debug, PartialEq)]
struct LargeArcFlag(bool);

#[derive(Debug, PartialEq)]
struct SweepFlag(bool);

impl Arc {
    fn svg_representation_values(&self, invert: bool) -> (Radius, LargeArcFlag, SweepFlag) {
        let Point([cx, cy]) = self.center;
        let Point([sx, sy]) = self.start;
        let radius = f64::hypot(cx - sx, cy - sy);

        if self.start == self.end {
            return (Radius(radius), LargeArcFlag(true), SweepFlag(false));
        } else if self.start == self.center {
            panic!()
        }

        let o0 = orient2d(self.start.0, self.center.0, self.end.0);
        let o1 = orient2d(self.start.0, [sx + sy - cy, sy + cx - sx], self.end.0);

        if o0 > 0. {
            if o1 > 0. {
                (
                    Radius(radius),
                    LargeArcFlag(!self.right),
                    SweepFlag(!self.right ^ invert),
                )
            } else if o1 < 0. {
                (
                    Radius(radius),
                    LargeArcFlag(!self.right),
                    SweepFlag(self.right ^ invert),
                )
            } else {
                panic!()
            }
        } else if o0 < 0. {
            if o1 > 0. {
                (
                    Radius(radius),
                    LargeArcFlag(self.right),
                    SweepFlag(!self.right ^ invert),
                )
            } else if o1 < 0. {
                (
                    Radius(radius),
                    LargeArcFlag(self.right),
                    SweepFlag(self.right ^ invert),
                )
            } else {
                panic!()
            }
        } else {
            if o1 > 0. {
                (
                    Radius(radius),
                    LargeArcFlag(false),
                    SweepFlag(!self.right ^ invert),
                )
            } else if o1 < 0. {
                (
                    Radius(radius),
                    LargeArcFlag(false),
                    SweepFlag(self.right ^ invert),
                )
            } else {
                (Radius(radius), LargeArcFlag(false), SweepFlag(false))
            }
        }
    }

    fn radius(&self) -> f64 {
        let Point([cx, cy]) = self.center;
        let Point([sx, sy]) = self.start;
        f64::hypot(sx - cx, sy - cy)
    }
}

impl SVGPath for Arc {
    fn svg_path_command(&self, invert_y: bool) -> String {
        let (Radius(radius), LargeArcFlag(large_arc_flag), SweepFlag(sweep_flag)) =
            self.svg_representation_values(invert_y);
        let laf = if large_arc_flag { '1' } else { '0' };
        let sf = if sweep_flag { '1' } else { '0' };
        let Point([x, y]) = self.end;
        format!("A {radius} {radius} 0 {laf} {sf} {x} {y} ")
    }

    fn length(&self) -> PathLength {
        let (_, LargeArcFlag(large_arc_flag), _) = self.svg_representation_values(false);
        let r = self.radius();
        let two_r = 2. * r;
        let Point([sx, sy]) = self.start;
        let Point([ex, ey]) = self.end;
        let s = f64::hypot(sx - ex, sy - ey);
        let short = if s < two_r {
            two_r * f64::asin(s / two_r)
        } else {
            r * std::f64::consts::PI
        };
        match large_arc_flag {
            true => PathLength(two_r - short),
            false => PathLength(short),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    mod arc_values {
        use super::*;

        #[test]
        fn case_1() {
            assert_eq!(
                (Arc {
                    start: Point([80., 80.]),
                    end: Point([125., 125.]),
                    center: Point([125., 80.]),
                    right: true,
                })
                .svg_representation_values(true),
                (Radius(45.), LargeArcFlag(false), SweepFlag(false))
            )
        }

        #[test]
        fn case_2() {
            assert_eq!(
                (Arc {
                    start: Point([230., 80.]),
                    end: Point([275., 125.]),
                    center: Point([230., 125.]),
                    right: true
                })
                .svg_representation_values(true),
                (Radius(45.), LargeArcFlag(true), SweepFlag(false))
            )
        }

        #[test]
        fn case_3() {
            assert_eq!(
                (Arc {
                    start: Point([80., 230.]),
                    end: Point([125., 275.]),
                    center: Point([80., 275.]),
                    right: false
                })
                .svg_representation_values(true),
                (Radius(45.), LargeArcFlag(false), SweepFlag(true))
            )
        }

        #[test]
        fn case_4() {
            assert_eq!(
                (Arc {
                    start: Point([230., 230.]),
                    end: Point([275., 275.]),
                    center: Point([275., 230.]),
                    right: false
                })
                .svg_representation_values(true),
                (Radius(45.), LargeArcFlag(true), SweepFlag(true))
            )
        }
    }
}
