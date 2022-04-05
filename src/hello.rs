use std::fmt;
use std::fmt::Formatter; // Import the `fmt` module.

// Define a structure named `List` containing a `Vec`.
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        let vec = &self.0;

        for (index, value) in vec.iter().enumerate() {
            if index != 0 { write!(f, ", ")?; }
            write!(f, "{}:{}", index, value)?;
        }
        write!(f, "]")
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "RGB ({red}, {green}, {blue}) 0x{red:0>2X}{green:0>2X}{blue:0>2X}",
               red = self.red, green = self.green, blue = self.blue)
    }
}
pub fn joe_format_print() {
    // pi
    let pi = 3.1415926;

    println!("pi = {:.3}", pi);
    println!("pi = {:.*}", 3, pi);
    println!("pi = {pi:.prec$}", pi = pi, prec = 3);
    println!("pi = {:.1$}", pi, 3);

    // fmt
    let v = List(vec![1,2,3]);
    println!("{}", v);

    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ].iter() {
        println!("{}", color);
    }
}