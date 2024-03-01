use crate::vector2::*;
use crate::signeddistance::*;
use crate::edgecolor::*;

struct EdgeSegment {
    pub color: EdgeColor,
}

impl Default for EdgeSegment {
    fn default() -> EdgeSegment {
        EdgeSegment {
            color: EdgeColor::WHITE,
        }
    }
}

impl EdgeSegment {
    pub fn create_linear(p0: Point2, p1: Point2, edge_color: EdgeColor) -> EdgeSegment {}

    pub fn create_quad_bez(p0: Point2, p1: Point2, p2: Point2, edge_color: EdgeColor) -> EdgeSegment {}

    pub fn create_cubic_bez(p0: Point2, p1: Point2, p2: Point2, p3: Point2, edge_color: EdgeColor) -> EdgeSegment {}
    pub fn create_with_color(ec: EdgeColor) -> EdgeSegment {
        EdgeSegment { color: ec }
    }
}


trait EdgeSegment {
    pub color: EdgeColor;

    pub fn create_linear(p0: Point2, p1: Point2, edge_color: EdgeColor) -> EdgeSegment;
    pub fn create_quad_bez(p0: Point2, p1: Point2, p2: Point2, edge_color: EdgeColor) -> EdgeSegment;
    pub fn create_cubic_bez(p0: Point2, p1: Point2, p2: Point2, p3: Point2, edge_color: EdgeColor) -> EdgeSegment;
    pub fn create_with_color(ec: EdgeColor) -> EdgeSegment;
}
