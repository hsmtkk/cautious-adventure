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

trait LineCalculator {
    fn calc_line(&self) -> f64;
}

struct Line {
    length: f64,
}

impl Line {
    fn new(length:f64) -> Self {
        Line{length}
    }
}

impl LineCalculator for Line {
    fn calc_line(&self) -> f64 {
        self.length
    }
}

impl LineCalculator for Rectangle {
    fn calc_line(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
}

#[cfg(test)]
mod tests {
    fn area<T: super::AreaCalculator>(x:&T) -> f64 {
        x.calc_area()
    }

    fn length<T: super::LineCalculator>(x:&T) -> f64 {
        x.calc_line()
    }

    #[test]
    fn test0(){
        let r = super::Rectangle::new(3.0, 4.0);
        assert_eq!(12.0, area(&r));
        assert_eq!(14.0, length(&r));
    }
}