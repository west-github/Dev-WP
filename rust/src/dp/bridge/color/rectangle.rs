use super::{red::Red, Shape};

pub struct Rectangle {
    height: i32,
    width: i32,
}

impl Rectangle {
    pub fn new(height: i32, width: i32) -> Self {
        Self { height, width }
    }
}

impl Shape for Rectangle {
    type Color = Red;

    fn set_color(&self) -> Self::Color {
        Red::new()
    }

    fn area(&self) {
        let res = self.height * self.width;
        self.set_color();
        println!("Printing with set color: {}", res);
    }
}
