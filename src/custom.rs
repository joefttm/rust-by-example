use std::fmt::Error;

struct Point {
    x: f32,
    y: f32,
}

#[allow(dead_code)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

impl Rectangle {
    // note: use static lifetime here
    pub fn new(x1: f32, y1: f32, x2: f32 ,y2 : f32) -> Result<Rectangle, &'static str> {
        let invalid_matrix = x1 >= x2 || y1 >= y2;
        match invalid_matrix {
            true => Err("invalid matrix"),
            false => Ok(Rectangle{
                top_left: Point{x:x1, y:y1},
                bottom_right: Point{x:x2, y:y2}
            })
        }
    }

    pub fn rect_area(&self) -> f32 {
        let Rectangle{top_left, bottom_right} = &self;
        let Point{x: x1, y:y1} = &top_left;
        let Point{x: x2, y:y2} = &bottom_right;
        (x2 - x1) * (y2 - y1)
    }
}

pub fn joe_test_custom_type() {
    // Instantiate a `Point`
    let point: Point = Point { x: 10.3, y: 0.4 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = Point { x: 5.2, ..point };

    // `bottom_right.y` will be the same as `point.y` because we used that field
    // from `point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    let Point {x: left_edge, y: top_edge} = point;

    let matrix = Rectangle::new(0.2, 0.3, 0.9, 1.2).unwrap();
    println!("matrix area: {}", matrix.rect_area());
}