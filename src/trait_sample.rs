trait AreaCalculator {
    fn calc_area(&self) -> f64;
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn new(width:f64, height:f64) -> Self {
        Rectangle{width, height}
    }
}

impl AreaCalculator for Rectangle {
    fn calc_area(&self) -> f64 {
        self.width * self.height
    }
}

#[cfg(test)]
mod tests {
    fn area<T: super::AreaCalculator>(x:&T) -> f64 {
        x.calc_area()
    }

    #[test]
    fn test0(){
        let r = super::Rectangle::new(3.0, 4.0);
        let want = 12.0;
        let got = area(&r);
        assert_eq!(want, got);
    }
}