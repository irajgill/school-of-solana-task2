mod shapes;
mod calculator;

use shapes::{Shape, Rectangle, Circle, compare_areas, find_largest_shape};
use calculator::{Calculator, Operation};


fn main() {
    println!("=== Task 2: Shapes and Calculator Demo ===\n");
    
    // Part 1: Shapes Demo
    shapes_demo();
    
    println!("\n{}\n", "=".repeat(50));

    
    // Part 2: Calculator Demo
    calculator_demo();
}

fn shapes_demo() {
    println!("🔷 PART 1: SHAPES DEMONSTRATION 🔷\n");
    
    // Create shapes with validation
    println!("Creating shapes with validation:");
    
    match Rectangle::new(5.0, 3.0) {
        Ok(rect) => {
            println!("✅ Rectangle created: {}x{}", rect.width(), rect.height());
            rect.display_info();
            println!("Is square? {}", rect.is_square());
        }
        Err(e) => println!("❌ Failed to create rectangle: {}", e),
    }
    
    match Circle::new(4.0) {
        Ok(circle) => {
            println!("\n✅ Circle created with radius: {}", circle.radius());
            circle.display_info();
            println!("Diameter: {:.2}", circle.diameter());
        }
        Err(e) => println!("❌ Failed to create circle: {}", e),
    }
    
    // Demonstrate error handling
    println!("\n--- Error Handling Demo ---");
    match Rectangle::new(-2.0, 3.0) {
        Ok(_) => println!("Rectangle created"),
        Err(e) => println!("❌ Expected error: {}", e),
    }
    
    match Circle::new(0.0) {
        Ok(_) => println!("Circle created"),
        Err(e) => println!("❌ Expected error: {}", e),
    }
    
    // Shape comparison
    println!("\n--- Shape Comparison ---");
    if let (Ok(rect), Ok(circle)) = (Rectangle::new(4.0, 3.0), Circle::new(2.0)) {
        match compare_areas(&rect, &circle) {
            std::cmp::Ordering::Greater => println!("Rectangle has larger area"),
            std::cmp::Ordering::Less => println!("Circle has larger area"),
            std::cmp::Ordering::Equal => println!("Rectangle and circle have equal areas"),
        }
        
        // Using trait objects
        let shapes: Vec<Box<dyn Shape>> = vec![
            Box::new(rect),
            Box::new(circle),
            Box::new(Rectangle::new(2.0, 2.0).unwrap()),
        ];
        
        if let Some(largest) = find_largest_shape(&shapes) {
            println!("Largest shape:");
            largest.display_info();
        }
    }
}

fn calculator_demo() {
    println!("🧮 PART 2: CALCULATOR DEMONSTRATION 🧮\n");
    
    let mut calc = Calculator::new();
    
    println!("Starting calculator with value: {}", calc.current_value());
    
    // Basic operations
    println!("\n--- Basic Operations ---");
    calc.set_value(10.0);
    
    match calc.add(5.0) {
        Ok(result) => println!("10 + 5 = {}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    match calc.multiply(3.0) {
        Ok(result) => println!("15 * 3 = {}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    match calc.divide(9.0) {
        Ok(result) => println!("45 / 9 = {}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    // Error handling
    println!("\n--- Error Handling ---");
    match calc.divide(0.0) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("❌ Expected error: {}", e),
    }
    
    // Using operation enum
    println!("\n--- Using Operation Enum ---");
    calc.set_value(8.0);
    match calc.perform_operation(Operation::Power, 2.0) {
        Ok(result) => println!("8^2 = {}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    match calc.perform_operation(Operation::Modulo, 3.0) {
        Ok(result) => println!("64 % 3 = {}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    // History demonstration
    println!("\n--- History Tracking ---");
    println!("Calculator history ({} entries):", calc.history_size());
    for (i, entry) in calc.get_history().iter().enumerate() {
        println!("  {}. {}", i + 1, entry);
    }
    
    // Undo operation
    println!("\n--- Undo Operation ---");
    println!("Current value before undo: {}", calc.current_value());
    match calc.undo() {
        Ok(value) => println!("After undo: {}", value),
        Err(e) => println!("Undo error: {}", e),
    }
    
    // Overflow handling demonstration
    println!("\n--- Overflow Handling ---");
    calc.set_value(f64::MAX);
    match calc.multiply(2.0) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("❌ Expected overflow: {}", e),
    }
}
