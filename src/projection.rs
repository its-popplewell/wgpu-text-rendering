use crate::vector2::*;

struct Projection {
    scale: Vector2,
    translate: Vector2
}

impl Default for Projection {
    fn default() -> Self {
        Self {
            scale: Vector2::new_splat(1.0),
            translate: Vector2::new_splat(0.0)
        }
    }
}

impl Projection {
    fn new(scale: &Vector2, translate: &Vector2) -> Self {
        Self {
            scale: scale.clone(),
            translate: translate.clone()
        }
    }

    // Converts the shape coordinate to pixel coordinate
    fn project(&self, coord: Point2) -> Point2 {
        self.scale * (coord + self.translate)
    }

    // Converts the pixel coordinate to shape coordinate
    fn unproject(&self, coord: Point2) -> Point2 {
        (coord / self.scale) - self.translate
    }

    // Converts the vector to pixel coordinate space
    fn project_vector(&self, vector: &Vector2) -> Vector2 {
        self.scale * *vector
    }

    // converts the vector from pixel coordinate space 
    fn unproject_vector(&self, vector: &Vector2) -> Vector2 {
        *vector / self.scale
    }

    // Converts the x-coordinate from shape to pixel coordinate space
    fn project_x(&self, x: f64) -> f64 {
        self.scale.x * (x + self.translate.x)
    }

    // Converts the y-coordinate from shape to pixel coordinate space
    fn project_y(&self, y: f64) -> f64 {
        self.scale.y * (y + self.translate.y)
    }

    // Converts the x-coordinate from pixel to shape coordinate space
    fn unproject_x(&self, x: f64) -> f64 {
        (x / self.scale.x) - self.translate.x
    }

    // Converts the y-coordinate from pixel to shape coordinate space
    fn unproject_y(&self, y: f64) -> f64 {
        (y / self.scale.y) - self.translate.y
    }
}


// GARBAGE tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_projection() {
        let p = Projection::default();
        let coord = Point2::new(6.0, 7.0);

        assert_eq!(p.project(coord), coord);
        assert_eq!(p.unproject(coord), coord);
    }

    #[test]
    fn test_project_vector() {
        let p = Projection::default();
        let vector = Vector2::new(4.0, 3.0);

        assert_eq!(p.project_vector(&vector), vector);
        assert_eq!(p.unproject_vector(&vector), vector);
    }

    #[test]
    fn test_project_individuals() {
        let p = Projection::new(
            &Vector2::new(2.0, 3.0), // scale
            &Vector2::new(0.0, 0.0)  // translate
        );

        assert_eq!(p.project_x(10.0), 20.0);
        assert_eq!(p.project_y(11.0), 33.0);
        assert_eq!(p.unproject_x(12.0), 6.0);
        assert_eq!(p.unproject_y(15.0), 5.0);
    }
}
