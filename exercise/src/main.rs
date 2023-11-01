use std::{f64::consts::PI, ops::Add, slice::Iter};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Point { x, y }
    }

    fn magnitude(&self) -> f64 {
        self.dist(Point::default())
    }

    fn dist(&self, p: Point) -> f64 {
        let x = self.x - p.x;
        let y = self.y - p.y;
        let sum = x * x + y * y;
        (sum as f64).sqrt()
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

pub struct Polygon {
    points: Vec<Point>,
}

impl Polygon {
    fn new() -> Self {
        Polygon { points: Vec::new() }
    }

    fn add_point(&mut self, p: Point) {
        self.points.push(p)
    }

    fn left_most_point(&self) -> Option<Point> {
        self.points.iter().min_by_key(|p| p.x).copied()
    }

    fn iter(&self) -> Iter<Point> {
        self.points.iter()
    }

    fn perimeter(&self) -> f64 {
        if self.points.len() <= 1 {
            return 0.0;
        }

        if self.points.len() == 2 {
            let first = self.points[0];
            let last = self.points[1];
            return first.dist(last);
        }

        let mut perimeter = 0.0;
        let mut prev_point = self.points.first().copied().unwrap();
        for point in self.points.iter().skip(1).copied() {
            perimeter += prev_point.dist(point);
            prev_point = point;
        }

        perimeter += self.points[0].dist(self.points[self.points.len() - 1]);
        perimeter
    }
}

pub struct Circle {
    center: Point,
    radius: usize,
}

impl Circle {
    fn new(center: Point, radius: usize) -> Self {
        Circle { center, radius }
    }

    fn perimeter(&self) -> f64 {
        2.0 * (self.radius as f64) * PI
    }
}

pub enum Shape {
    Polygon(Polygon),
    Circle(Circle),
}

impl Shape {
    fn perimeter(&self) -> f64 {
        match self {
            Shape::Polygon(p) => p.perimeter(),
            Shape::Circle(c) => c.perimeter(),
        }
    }
}

impl From<Polygon> for Shape {
    fn from(value: Polygon) -> Self {
        Shape::Polygon(value)
    }
}

impl From<Circle> for Shape {
    fn from(value: Circle) -> Self {
        Shape::Circle(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn round_two_digits(x: f64) -> f64 {
        (x * 100.0).round() / 100.0
    }

    #[test]
    fn test_point_magnitude() {
        let p1 = Point::new(12, 13);
        assert_eq!(round_two_digits(p1.magnitude()), 17.69);
    }

    #[test]
    fn test_point_dist() {
        let p1 = Point::new(10, 10);
        let p2 = Point::new(14, 13);
        assert_eq!(round_two_digits(p1.dist(p2)), 5.00);
    }

    #[test]
    fn test_point_add() {
        let p1 = Point::new(16, 16);
        let p2 = p1 + Point::new(-4, 3);
        assert_eq!(p2, Point::new(12, 19));
    }

    #[test]
    fn test_polygon_left_most_point() {
        let p1 = Point::new(12, 13);
        let p2 = Point::new(16, 16);

        let mut poly = Polygon::new();
        poly.add_point(p1);
        poly.add_point(p2);
        assert_eq!(poly.left_most_point(), Some(p1));
    }

    #[test]
    fn test_polygon_iter() {
        let p1 = Point::new(12, 13);
        let p2 = Point::new(16, 16);

        let mut poly = Polygon::new();
        poly.add_point(p1);
        poly.add_point(p2);

        let points = poly.iter().cloned().collect::<Vec<_>>();
        assert_eq!(points, vec![Point::new(12, 13), Point::new(16, 16)]);
    }

    #[test]
    fn test_shape_perimeters() {
        let mut poly = Polygon::new();
        poly.add_point(Point::new(12, 13));
        poly.add_point(Point::new(17, 11));
        poly.add_point(Point::new(16, 16));
        let shapes = vec![
            Shape::from(poly),
            Shape::from(Circle::new(Point::new(10, 20), 5)),
        ];
        let perimeters = shapes
            .iter()
            .map(Shape::perimeter)
            .map(round_two_digits)
            .collect::<Vec<_>>();
        assert_eq!(perimeters, vec![15.48, 31.42]);
    }
}

#[allow(dead_code)]
fn main() {}
