mod shapes;
mod calculator;

#[cfg(test)]
mod tests;

use shapes::{Rectangle, Circle, Shape};
use calculator::{Calculator, CalculatorError};

fn main() {
    println!("=== Shapes Demo ===");
    
    // Rectangle demo
    match Rectangle::new(5.0, 3.0) {
        Ok(rect) => {
            println!("{} → area: {:.2}, perimeter: {:.2}", 
                rect, rect.area(), rect.perimeter());
        }
        Err(e) => println!("Rectangle error: {}", e),
    }
    
    // Circle demo
    match Circle::new(2.5) {
        Ok(circ) => {
            println!("{} → area: {:.2}, perimeter: {:.2}", 
                circ, circ.area(), circ.perimeter());
        }
        Err(e) => println!("Circle error: {}", e),
    }

    // Polymorphism demo
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Rectangle::new(4.0, 6.0).unwrap()),
        Box::new(Circle::new(3.0).unwrap()),
    ];
    
    println!("\n=== Polymorphism Demo ===");
    for (i, shape) in shapes.iter().enumerate() {
        println!("Shape {}: {} - Area: {:.2}", 
            i + 1, shape.name(), shape.area());
    }

    println!("\n=== Calculator Demo ===");
    let mut calc = Calculator::new();
    
    // Basic operations
    if let Err(e) = calc.add(10) {
        println!("Error: {}", e);
        return;
    }
    
    calc.multiply(3).unwrap();
    calc.subtract(5).unwrap();
    
    println!("Final result: {}", calc.current_value());
    println!("\nHistory:");
    println!("{}", calc.history_as_string());
    
    // Error handling demo
    println!("\n=== Error Handling Demo ===");
    match calc.divide(0) {
        Ok(_) => println!("Division succeeded"),
        Err(e) => println!("Division error: {}", e),
    }
    
    // Factorial demo
    calc.clear();
    calc.add(5).unwrap();
    match calc.factorial() {
        Ok(result) => println!("5! = {}", result),
        Err(e) => println!("Factorial error: {}", e),
    }
    
    println!("\nRun `cargo test` to execute all tests!");
}
