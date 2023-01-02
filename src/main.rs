#[derive(Default)]
struct Square<T> 
{
    side : T,
}

impl Square<u32> {
    fn new( t: u32) -> Self {
        Square { side : t}
    }

    fn area(&self) -> u32 {
        return self.side * self.side
    }
}

impl Square<f64> {
    fn new( t: f64) -> Self {
        Square { side : t}
    }

    fn area(&self) -> f64 {
        return self.side * self.side
    }
}

impl Square<String> {
    fn new( t: &str) -> Self {

        Square { side : t.to_string()}
    }

    fn area(&self) -> f64 {
        return self.side.parse::<f64>().unwrap() * self.side.parse::<f64>().unwrap()
    }
}

struct Triangle<T>{
    height:T,
    base:T,
    
}

impl Triangle<f64>{
    fn new( t: f64, t1: f64) -> Self {
        Triangle { height : t, base : t1}
    }

    fn area(&self) -> f64 {
        return self.height * self.base / 2.0
    }
}

struct Pyramid<T, U>{
    base:T,
    height:U,
}

impl Pyramid<Square<u32>,f64>{
    fn new( t: Square<u32>, t1: f64) -> Self {
        Pyramid { base : t, height : t1}
    }

    fn volume(&self) -> f64 {
        return self.base.area() as f64 * (self.height / 3.0)
    }
}

impl Pyramid<Triangle<f64>,f64>{
    fn new( t: Triangle<f64>, t1: f64) -> Self {
        Pyramid { base : t, height : t1}
    }

    fn volume(&self) -> f64 {
        return self.base.area() * self.height / 3.0
    }
}


fn main() {
    let square = Square::<u32>::new(5);
    let square_float = Square::<f64>::new(5.4);
    let square_string = Square::<String>::new("6");

    println!("square area is {}", square.area());
    println!("square_float area is {}", square_float.area());
    println!("square_string area is {}", square_string.area());

    let triangle = Triangle::new(14.9, 20.1);
    let pyramid_square = Pyramid::<Square<u32>, f64>::new(square, 24.3);
    let pyramid_triangle = Pyramid::<Triangle<f64>, f64>::new(triangle, 24.3);

    println!("pyramid_square volume is {}", pyramid_square.volume());
    println!("pyramid_triangle volume is {}", pyramid_triangle.volume());
}
