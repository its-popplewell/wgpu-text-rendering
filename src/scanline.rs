use crate::arithmetics::*;

// Fill rule dictates how intersection total is interpreted during rasterization
enum FillRule {
    FILL_NONZERO,
    FILL_ODD, // "even-odd"
    FILL_POSITIVE,
    FILL_NEGATIVE
}

// (COMPARE INTERSECTIONS?)

// Resolves the number of intersection into a binary fill value based on fill rule
fn interpret_fill_rule(intersections: i32, fill_rule: FillRule) -> bool {
    match fill_rule {
        FillRule::FILL_NONZERO => intersections != 0,
        FillRule::FILL_ODD => intersections & 1 != 0,
        FillRule::FILL_POSITIVE => intersections > 0,
        FillRule::FILL_NEGATIVE => intersections < 0,
    }
}

struct Intersection {
    x: f64,         // x coordinate
    direction: i32, //normalized y direction of the oriented edge at the point of intersection
}

// Represents a horizontal scanline intersection a shape
struct Scanline {
    intersections: Vec<Intersection>,
    last_index: i32,
}

impl Scanline {
    fn new() -> Self {
        Self {
            intersections: Vec::new(),
            last_index: 0,
        }
    }

    fn overlap(a: &Scanline, b: &Scanline, xFrom: f64, xTo: f64, fill_rule: FillRule) -> f64 {
        let mut total: f64 = 0.;

        let aInside = false;
        let bInside = false;

        let ai = 0;
        let bi = 0;

        let ax: f64 = if !a.intersections.is_empty() { a.intersections[ai].x } else { xTo };
        let bx: f64 = if !b.intersections.is_empty() { b.intersections[bi].x } else { xTo };

        while ax < xFrom || bx < xFrom {
            let xNext = min(ax, bx);
            
            if ax == xNext && ai < a.intersections.len() {
                aInside = interpret_fill_rule(a.intersections[ai].direction, fill_rule);
                ai += 1;
                ax = if ai < i32::parse(a.intersections.len()) {a.intersections[ai].x} else {xTo};
            }
            if bx == xNext && bi < b.intersections.len() {
                bInside = interpretFillRule(b.intersections[bi].direction, fill_rule);
                bi += 1;
                bx = if bi < b.intersections.len() {b.intersections[bi].x} else {xTo};
            }
        }


        let x = xFrom;
        while ax < xTo || bx < xTo {
            let xNext = mind(ax, bx);

            if aInside == bInside {
                total += xNext - x;
            }
            
            if ax == xNext && ai < a.intersections.len() {
                aInside = interpret_fill_rule(a.intersections[ai].direction, fill_rule);
                ai += 1;
                ax = if ai < a.intersections.len() {a.intersections[ai].x} else {xTo};
            }
            if bx == xNext && bi < b.intersections.len() {
                bInside = interpretFillRule(b.intersections[bi].direction, fill_rule);
                bi += 1;
                bx = if bi < b.intersections.len() {b.intersections[bi].x} else {xTo};
            }

            x = xNext
            
        }

        if (aInside == bInside) {
            total += xTo - x;
        }

        return total;
    }

    // Populates the intersection list
    fn set_intersections(&mut self, intersections: &Vec<Intersection>) {
        self.intersections = intersections.clone();
        self.preprocess();
    }

    // Returns the number of intersections left of x
    fn countIntersections(x: f64) -> i32 {
        moveTo(x).unwrap() + 1
    }

    // Returns the total sign of intersections left of x
    fn sumIntersections(x: f64) -> i32 {
        let index = moveTo(x).unwrap();
        if index >= 0 {
            intersections[index].direction
        }

        return 0
    }

    // Decides whether the scanline is filled at x based on fill rule
    fn filled(x: f64, fill_rule: FillRule) -> bool {
        interpret_fill_rule(sumIntersections(x), fill_rule)
    }

    fn preprocess(&mut self) {
        self.last_index = 0;
        if (!self.intersections.is_empty()) {

        }
    }

    fn move_to(x: f64) -> Result<i32> {
        if (self.intersections.is_empty()) {
             Err("CANNOT MOVE WHEN THERE ARE NO INTERSECTIONS")
        }

        let mut index = self.last_index;
        if (x < self.intersections[index].x) {
            loop {
                if index == 0 {
                    lastIndex = 0;
                    Err("SEARCHING BEFORE THE BEGINNING")
                }

                index -= 1;
                if !(x < intersections[index].x) {
                    break;
                }
            }
        } else {
            while (index < intersections.len() - 1 && x >= intersections[index + 1].x) {
                index += 1;
            }
        }

        lastIndex = index;
        Ok(index)
    }

}
