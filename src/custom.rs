use std::fmt::Error;
use crate::custom::List::*;

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

enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click{x: u64, y:u64},
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        // Destructure `c` from inside the `enum`.
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        // Destructure `Click` into `x` and `y`.
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y);
        },
    }
}


enum List {
    Cons(u32, Box<List>),
    Nil,
}

impl List {
    fn new() -> List {
        // `Nil` has type `List`
        Nil
    }

    fn prepend(self, element: u32) -> List {
        Cons(element, Box::new(self))
    }

    fn len(&self) -> u32 {
        match *self {
            Cons(_, ref tail) => tail.len() + 1,
            Nil => 0,
        }
    }

    // Return representation of the list as a (heap allocated) string
    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                // `format!` is similar to `print!`, but returns a heap
                // allocated string instead of printing to the console
                format!("{}, {}", head, tail.stringify())
            },
            Nil => {
                format!("Nil")
            },
        }
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

    let mut list = List::new();
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(4);

    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());

    let a = [1,2,6];
}