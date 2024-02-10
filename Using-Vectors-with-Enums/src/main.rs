

#[derive(Debug)]
enum Shape {
    Circle(f64),
    Square(f64),
}


fn max_area(shapes: &Vec<Shape>) -> usize {
    let mut max_shape = shapes
    .iter()
    .map(|shape| match shape {
        Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
        Shape::Square(length) => length * length,
    });
    let max = max_shape.clone().fold(f64::NAN, f64::max);
    let index: usize = max_shape.position(|x| x==max).unwrap();


    return index;
}


fn main() {
    let shape = vec![Shape::Circle(5.0), Shape::Square(10.0)];

    let total_area: f64 = shape
    .iter()
    .map(|shape| match shape {
        Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
        Shape::Square(length) => length * length,
    })
    .sum();

    println!("{:?}",shape[max_area(&shape)])
}
