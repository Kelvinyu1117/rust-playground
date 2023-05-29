// TODO: remove this when you're done with your implementation.
#![allow(unused_variables, dead_code)]

use std::cmp;
use std::f64::consts::PI;
use std::ops;
use std::slice::Iter;

#[derive(Debug, Copy, Clone, cmp::Eq)]
pub struct Point {
    // add fields
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    pub fn dist(&self, that: Point) -> f64 {
        (((self.x - that.x).pow(2) + (self.y - that.y).pow(2)) as f64).sqrt()
    }

    pub fn dist_ref(&self, that: &Point) -> f64 {
        (((self.x - that.x).pow(2) + (self.y - that.y).pow(2)) as f64).sqrt()
    }

    pub fn magnitude(&self) -> f64 {
        ((self.x * self.x + self.y * self.y) as f64).sqrt()
    }
}

impl ops::Add for Point {
    type Output = Point;
    fn add(self, that: Point) -> Self::Output {
        Point {
            x: self.x + that.x,
            y: self.y + that.y,
        }
    }
}

impl cmp::PartialEq for Point {
    fn eq(&self, that: &Self) -> bool {
        self.x == that.x && self.y == that.y
    }
}

#[derive(Debug)]
pub struct Polygon {
    // add fields
    points: Vec<Point>,
}

impl Polygon {
    pub fn new() -> Self {
        Polygon { points: vec![] }
    }

    pub fn add_point(&mut self, pt: Point) {
        self.points.push(pt);
    }

    pub fn left_most_point(&self) -> Option<Point> {
        if self.points.is_empty() {
            None
        } else {
            let mut idx = 0;
            for i in 0..self.points.len() {
                if self.points[i].x < self.points[idx].x {
                    idx = i;
                }
            }

            return Some(self.points[idx]);
        }
    }

    pub fn iter(&self) -> Iter<Point> {
        return self.points.iter();
    }
}
#[derive(Debug)]
pub struct Circle {
    centre: Point,
    radius: i32,
}

impl Circle {
    // add methods
    pub fn new(centre: Point, radius: i32) -> Self {
        Circle { centre, radius }
    }
}

#[derive(Debug)]
pub enum Shape {
    Polygon(Polygon),
    Circle(Circle),
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
impl Shape {
    pub fn perimeter(&self) -> f64 {
        match self {
            Shape::Polygon(state) => {
                let mut dists: Vec<f64> = Vec::new();
                let points = state.iter().collect::<Vec<_>>();

                for i in 0..points.len() {
                    dists.push(points[i].dist_ref(points[(i + 1) % points.len()]))
                }

                dists.iter().sum()
            }
            Shape::Circle(state) => 2.0 * PI * state.radius as f64,
        }
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
