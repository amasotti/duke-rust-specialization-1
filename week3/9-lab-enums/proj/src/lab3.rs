enum Shape {
    Circle(f64),
    Square(f64),
    Triangle(f64, f64),
}

fn get_area(shape: &Shape) -> f64 {
    match shape {
        Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
        Shape::Square(length) => length * length,
        Shape::Triangle(base, height) => (base * height) / 2.0,
    }
}

fn get_largest_shape_by_area(shapes: &Vec<Shape>) -> &Shape {
    shapes
        .iter()
        .max_by(|a, b| {
            
            let area_a = get_area(a);
            let area_b = get_area(b);
            area_a.partial_cmp(&area_b).unwrap_or(std::cmp::Ordering::Equal)
        })
        .expect("No shapes found")
}


pub fn lab3() {
    let shapes = vec![Shape::Circle(1.0), Shape::Square(3.0), Shape::Triangle(4.0, 3.0)];

    let total_area: f64 = shapes
        .iter()
        .map(|shape| match shape {
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
            Shape::Square(length) => length * length,
            Shape::Triangle(base, height) => (base * height) / 2.0,
        })
        .sum();

    println!("Total area: {:.3} sq. units", total_area);
    
    let largest_shape = get_largest_shape_by_area(&shapes);
    match largest_shape {
        Shape::Circle(radius) => println!("Largest shape is a circle with radius {:.3}", radius),
        Shape::Square(length) => println!("Largest shape is a square with side length {:.3}", length),
        Shape::Triangle(base, height) => println!("Largest shape is a triangle with base {:.3} and height {:.3}", base, height),
    }
}
