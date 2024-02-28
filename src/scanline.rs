use crate::arithmetics::*;

// Fill rule dictates how intersection total is interpreted during rasterization
#[derive(Copy, Clone)]
enum FillRule {
    FILL_NONZERO,
    FILL_ODD, // "even-odd"
    FILL_POSITIVE,
    FILL_NEGATIVE
}

// (COMPARE INTERSECTIONS?)
use std::cmp::Ordering;
fn i8_to_ordering(i: i8) -> Ordering {
    if i < 0 {
        return Ordering::Less;
    } else if i > 0 {
        return Ordering::Greater;
    } else {
        return Ordering::Equal;
    }
}

// Resolves the number of intersection into a binary fill value based on fill rule
fn interpret_fill_rule(intersections: i32, fill_rule: FillRule) -> bool {
    match fill_rule {
        FillRule::FILL_NONZERO => intersections != 0,
        FillRule::FILL_ODD => intersections & 1 != 0,
        FillRule::FILL_POSITIVE => intersections > 0,
        FillRule::FILL_NEGATIVE => intersections < 0,
    }
}

#[derive(PartialEq, PartialOrd)]
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

        let mut aInside = false;
        let mut bInside = false;

        let mut ai = 0;
        let mut bi = 0;

        let mut ax: f64 = if !a.intersections.is_empty() { a.intersections[ai].x } else { xTo };
        let mut bx: f64 = if !b.intersections.is_empty() { b.intersections[bi].x } else { xTo };

        while ax < xFrom || bx < xFrom {
            let xNext = min(ax, bx);
            
            if ax == xNext && ai < a.intersections.len() {
                aInside = interpret_fill_rule(a.intersections[ai].direction, fill_rule);
                ai += 1;
                ax = if ai < a.intersections.len() {a.intersections[ai].x} else {xTo};
            }
            if bx == xNext && bi < b.intersections.len() {
                bInside = interpret_fill_rule(b.intersections[bi].direction, fill_rule);
                bi += 1;
                bx = if bi < b.intersections.len() {b.intersections[bi].x} else {xTo};
            }
        }


        let mut x = xFrom;
        while ax < xTo || bx < xTo {
            let xNext = min(ax, bx);

            if aInside == bInside {
                total += xNext - x;
            }
            
            if ax == xNext && ai < a.intersections.len() {
                aInside = interpret_fill_rule(a.intersections[ai].direction, fill_rule);
                ai += 1;
                ax = if ai < a.intersections.len() {a.intersections[ai].x} else {xTo};
            }
            if bx == xNext && bi < b.intersections.len() {
                bInside = interpret_fill_rule(b.intersections[bi].direction, fill_rule);
                bi += 1;
                bx = if bi < b.intersections.len() {b.intersections[bi].x} else {xTo};
            }

            x = xNext
            
        }

        if aInside == bInside {
            total += xTo - x;
        }

        return total;
    }

    // Populates the intersection list
    fn set_intersections(&mut self, intersections: Vec<Intersection>) {
        self.intersections = intersections;
        self.preprocess();
    }

    // Returns the number of intersections left of x
    fn countIntersections(&mut self, x: f64) -> i32 {
        self.move_to(x).unwrap() + 1
    }

    // Returns the total sign of intersections left of x
    fn sumIntersections(&mut self, x: f64) -> i32 {
        let index = self.move_to(x).unwrap();
        if index >= 0 {
            return self.intersections[index as usize].direction;
        }

        return 0
    }

    // Decides whether the scanline is filled at x based on fill rule
    fn filled(&mut self, x: f64, fill_rule: FillRule) -> bool {
        interpret_fill_rule(self.sumIntersections(x), fill_rule)
    }

    fn preprocess(&mut self) {
        self.last_index = 0;
        if !self.intersections.is_empty() {
            use quicksort::quicksort_by;
            quicksort_by(self.intersections.as_mut_slice(), |a, b| i8_to_ordering(sign(a.x - b.x)));
            
            let mut total_direction = 0;
            for inter in self.intersections.iter_mut() {
                total_direction += inter.direction;
                inter.direction = total_direction;
            }
            
        }
    }

    fn move_to(&mut self, x: f64) -> Option<i32> {
        if self.intersections.is_empty() {
             panic!("CANNOT MOVE WHEN THERE ARE NO INTERSECTIONS");
             return None;
        }

        let mut index = self.last_index;
        if x < self.intersections[index as usize].x {
            loop {
                if index == 0 {
                    self.last_index = 0;
                    panic!("SEARCHING BEFORE THE BEGINNING");
                    return None;
                }

                index -= 1;
                if !(x < self.intersections[index as usize].x) {
                    break;
                }
            }
        } else {
            while (index < (self.intersections.len() - 1) as i32 && x >= self.intersections[(index + 1) as usize].x) {
                index += 1;
            }
        }

        self.last_index = index;
        Some(index)
    }

}
