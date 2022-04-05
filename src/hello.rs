pub fn joe_format_print() {
    // pi
    let pi = 3.1415926;

    println!("pi = {:.3}", pi);
    println!("pi = {:.*}", 3, pi);
    println!("pi = {pi:.prec$}", pi = pi, prec = 3);
    println!("pi = {:.1$}", pi, 3);
}