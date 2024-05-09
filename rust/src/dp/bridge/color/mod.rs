mod rectangle;
mod red;

trait Color {
    fn color(&self);
}

trait Shape {
    type Color: Color;

    fn set_color(&self) -> Self::Color;

    fn print(&self) {
        for i in 1000..10010 {
            (i % 2 == 0).then(|| {
                for _ in 1..100 {
                    println!("Yes we got a good index let fucking go")
                }
            });
        }

        println!("Printing shape");
    }

    fn area(&self);
}

#[test]
fn test() {
    use self::rectangle::Rectangle;
    let shape = Rectangle::new(10, 20);

    shape.print();
    shape.area();
}
