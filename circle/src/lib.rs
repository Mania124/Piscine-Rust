use std::f64::consts::PI;

#[derive(Debug, Clone, Copy)]
pub struct Circle {
    pub center: Point,
    pub radius: f64,
}

impl Circle {
    pub fn diameter(&self) -> f64 {
        self.radius * 2.0
    }
    pub fn area(&self) -> f64 {
        PI * (self.radius.powi(2))
    }
    pub fn intersect(&self, other: Circle) -> bool {
        let dis = (self.center).distance(&other.center);
        if dis <= self.radius || dis <= other.radius {
            true
        } else {
            false
        }
    }
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            center: Point(x, y),
            radius: z,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Point(pub f64, pub f64);

impl Point {
    // Distance between t
    pub fn distance(&self, other: &Point) -> f64 {
        let dx = self.0 - other.0;
        let dy = self.1 - other.1;
        (dx.powi(2) + dy.powi(2)).sqrt()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let circle = Circle::new(500.0, 500.0, 150.0);
        let circle1 = Circle {
            center: Point(80.0, 115.0),
            radius: 30.0,
        };
        let result = circle.area();
        assert_eq!(result, 70685.83470577035);
        let result1 = circle.diameter();
        assert_eq!(result1, 300.0);
        let result2 = circle.intersect(circle1);
        assert_eq!(result2, false);
    }
}
