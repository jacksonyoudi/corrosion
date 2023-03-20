use std::fmt::{Display, Formatter, Result};


trait Area {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}


impl Display for Circle {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Circle radius:{}", self.radius)
    }
}

impl Area for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

struct Triangle {
    base: f64,
    height: f64,
}

impl Display for Triangle {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Triangle base:{}, height:{}", self.base, self.height)
    }
}


impl Area for Triangle {
    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }
}

struct Square {
    side: f64,
}

impl Area for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Square side:{}", self.side)
    }
}


fn print_area<T>(shape: &T)
    where T: Area + Display
{
    println!("The area of the shape is: {}", shape.area());
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn area_test() {
        let circle: Circle = Circle { radius: 5.0 };
        let triangle: Triangle = Triangle { base: 4.0, height: 3.0 };
        let square: Square = Square { side: 2.0 };

        print_area(&circle);
        print_area(&triangle);
        print_area(&square);
    }
}