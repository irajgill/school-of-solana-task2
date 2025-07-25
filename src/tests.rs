#[cfg(test)]
mod tests {
    use super::shapes::*;
    use super::calculator::*;
    use std::f64::consts::PI;

    // Shape tests
    #[test]
    fn test_rectangle_creation_valid() {
        let rect = Rectangle::new(5.0, 3.0).unwrap();
        assert_eq!(rect.width(), 5.0);
        assert_eq!(rect.height(), 3.0);
    }

    #[test]
    fn test_rectangle_creation_invalid() {
        assert!(Rectangle::new(-1.0, 3.0).is_err());
        assert!(Rectangle::new(5.0, 0.0).is_err());
        assert!(Rectangle::new(0.0, 0.0).is_err());
    }

    #[test]
    fn test_rectangle_area_and_perimeter() {
        let rect = Rectangle::new(4.0, 6.0).unwrap();
        assert_eq!(rect.area(), 24.0);
        assert_eq!(rect.perimeter(), 20.0);
    }

    #[test]
    fn test_rectangle_is_square() {
        let square = Rectangle::new(5.0, 5.0).unwrap();
        let rect = Rectangle::new(5.0, 3.0).unwrap();
        assert!(square.is_square());
        assert!(!rect.is_square());
    }

    #[test]
    fn test_circle_creation_valid() {
        let circle = Circle::new(3.0).unwrap();
        assert_eq!(circle.radius(), 3.0);
        assert_eq!(circle.diameter(), 6.0);
    }

    #[test]
    fn test_circle_creation_invalid() {
        assert!(Circle::new(-1.0).is_err());
        assert!(Circle::new(0.0).is_err());
    }

    #[test]
    fn test_circle_area_and_perimeter() {
        let circle = Circle::new(2.0).unwrap();
        let expected_area = PI * 4.0;
        let expected_perimeter = 2.0 * PI * 2.0;
        
        assert!((circle.area() - expected_area).abs() < 1e-10);
        assert!((circle.perimeter() - expected_perimeter).abs() < 1e-10);
    }

    #[test]
    fn test_shape_scaling() {
        let mut rect = Rectangle::new(2.0, 3.0).unwrap();
        let mut circle = Circle::new(2.0).unwrap();
        
        assert!(rect.scale(2.0).is_ok());
        assert_eq!(rect.width(), 4.0);
        assert_eq!(rect.height(), 6.0);
        
        assert!(circle.scale(1.5).is_ok());
        assert_eq!(circle.radius(), 3.0);
        
        assert!(rect.scale(-1.0).is_err());
        assert!(circle.scale(0.0).is_err());
    }

    // Calculator tests
    #[test]
    fn test_calculator_basic_operations() {
        let mut calc = Calculator::new();
        calc.set_value(10.0);
        
        assert_eq!(calc.add(5.0).unwrap(), 15.0);
        assert_eq!(calc.subtract(3.0).unwrap(), 12.0);
        assert_eq!(calc.multiply(2.0).unwrap(), 24.0);
        assert_eq!(calc.divide(4.0).unwrap(), 6.0);
    }

    #[test]
    fn test_calculator_division_by_zero() {
        let mut calc = Calculator::new();
        calc.set_value(10.0);
        
        match calc.divide(0.0) {
            Err(CalculatorError::DivisionByZero) => {}
            _ => panic!("Expected DivisionByZero error"),
        }
    }

    #[test]
    fn test_calculator_power_and_modulo() {
        let mut calc = Calculator::new();
        calc.set_value(3.0);
        
        assert_eq!(calc.power(3.0).unwrap(), 27.0);
        assert_eq!(calc.modulo(5.0).unwrap(), 2.0);
    }

    #[test]
    fn test_calculator_operation_enum() {
        let mut calc = Calculator::new();
        calc.set_value(8.0);
        
        assert_eq!(calc.perform_operation(Operation::Add, 2.0).unwrap(), 10.0);
        assert_eq!(calc.perform_operation(Operation::Multiply, 3.0).unwrap(), 30.0);
        assert_eq!(calc.perform_operation(Operation::Divide, 6.0).unwrap(), 5.0);
    }

    #[test]
    fn test_calculator_history() {
        let mut calc = Calculator::new();
        calc.set_value(5.0);
        
        calc.add(3.0).unwrap();
        calc.multiply(2.0).unwrap();
        
        assert_eq!(calc.history_size(), 2);
        assert_eq!(calc.last_result().unwrap(), 16.0);
        
        let history = calc.get_history();
        assert_eq!(history[0].result, 8.0);
        assert_eq!(history[1].result, 16.0);
    }

    #[test]
    fn test_calculator_undo() {
        let mut calc = Calculator::new();
        calc.set_value(10.0);
        
        calc.add(5.0).unwrap();
        assert_eq!(calc.current_value(), 15.0);
        
        calc.undo().unwrap();
        assert_eq!(calc.current_value(), 10.0);
        assert_eq!(calc.history_size(), 0);
    }

    #[test]
    fn test_calculator_clear() {
        let mut calc = Calculator::new();
        calc.set_value(42.0);
        calc.add(8.0).unwrap();
        
        calc.clear();
        assert_eq!(calc.current_value(), 0.0);
        
        calc.clear_history();
        assert_eq!(calc.history_size(), 0);
    }

    #[test]
    fn test_calculator_overflow_handling() {
        let mut calc = Calculator::new();
        calc.set_value(f64::MAX);
        
        match calc.add(f64::MAX) {
            Err(CalculatorError::Overflow) => {}
            _ => panic!("Expected Overflow error"),
        }
    }

    #[test]
    fn test_simple_calculator_function() {
        assert_eq!(calculate_simple(5.0, Operation::Add, 3.0).unwrap(), 8.0);
        assert_eq!(calculate_simple(10.0, Operation::Subtract, 4.0).unwrap(), 6.0);
        assert_eq!(calculate_simple(6.0, Operation::Multiply, 7.0).unwrap(), 42.0);
        assert_eq!(calculate_simple(15.0, Operation::Divide, 3.0).unwrap(), 5.0);
        
        match calculate_simple(10.0, Operation::Divide, 0.0) {
            Err(CalculatorError::DivisionByZero) => {}
            _ => panic!("Expected DivisionByZero error"),
        }
    }

    #[test]
    fn test_shape_comparison() {
        let rect = Rectangle::new(4.0, 3.0).unwrap();
        let circle = Circle::new(2.0).unwrap();
        
        // Rectangle area: 12, Circle area: π * 4 ≈ 12.57
        assert_eq!(compare_areas(&rect, &circle), std::cmp::Ordering::Less);
    }

    #[test]
    fn test_find_largest_shape() {
        let shapes: Vec<Box<dyn Shape>> = vec![
            Box::new(Rectangle::new(2.0, 3.0).unwrap()), // area: 6
            Box::new(Circle::new(2.0).unwrap()),          // area: ~12.57
            Box::new(Rectangle::new(5.0, 1.0).unwrap()), // area: 5
        ];
        
        let largest = find_largest_shape(&shapes).unwrap();
        assert_eq!(largest.name(), "Circle");
    }
}
