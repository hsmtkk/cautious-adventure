use std::cmp::Ordering;
use std::ops::Add;

struct Point2D{
    x: f64,
    y: f64,
}

impl Point2D {
    fn new(x:f64, y:f64) -> Self {
        Point2D {x,y }
    }

    fn distance_sq(&self) -> f64{
        self.x.powi(2) + self.y.powi(2)
    }
}

impl Add for Point2D {
    type Output =  Self;

    fn add(self, rhs: Self) -> Self::Output {
        Point2D::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl PartialEq for Point2D {
    fn eq(&self, other: &Self) -> bool {
        let dist_self_sq = self.distance_sq();
        let dist_another_sq = other.distance_sq();
        dist_self_sq.eq(&dist_another_sq)
    }
}

impl PartialOrd for Point2D {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let dist_self_sq = self.distance_sq();
        let dist_another_sq = other.distance_sq();
        dist_self_sq.partial_cmp(&dist_another_sq)
    }
}

#[cfg(test)]
mod tests {
    use super::Point2D;

    #[test]
    fn test(){
        let x = Point2D::new(3.0, 4.0);
        let y = Point2D::new(6.0, 8.0);
        let z = Point2D::new(4.0, 3.0);
        assert!(x < y);
        assert!(x == z);
    }
}
