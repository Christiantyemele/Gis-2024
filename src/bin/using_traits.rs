trait Area {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Area for Circle {
    fn area(&self) -> f64 {
        return 3.14 * self.radius;
    }
}

impl Area for Rectangle {
    fn area(&self) -> f64 {
        return self.width * self.height;
    }
}
fn main() {}

#[cfg(test)]
mod test {
    use crate::{Area, Circle, Rectangle};

    #[test]
    fn test_traits() {
        let width = 6.4;
        let height = 54.5;
        let radius = 5.0;

        let circle = Circle { radius };
        let rectangle = Rectangle { width, height };
        let area_circle = circle.area();
        let area_rectangle = rectangle.area();

        assert_eq!(area_circle, 3.14 * radius);
        assert_eq!(area_rectangle, width * height)
    }
}
