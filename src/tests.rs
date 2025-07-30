//! Comprehensive tests for shapes and calculator modules

use crate::shapes::{Rectangle, Circle, Shape, ShapeError};
use crate::calculator::{Calculator, CalculatorError};

// ===== SHAPES TESTS =====

#[cfg(test)]
mod shape_tests {
    use super::*;

    #[test]
    fn rectangle_creation_valid() {
        let rect = Rectangle::new(4.0, 3.0);
        assert!(rect.is_ok());
        let rect = rect.unwrap();
        assert_eq!(rect.width(), 4.0);
        assert_eq!(rect.height(), 3.0);
    }

    #[test]
    fn rectangle_creation_invalid() {
        assert!(matches!(Rectangle::new(-1.0, 3.0), Err(ShapeError::NegativeValue)));
        assert!(matches!(Rectangle::new(4.0, -1.0), Err(ShapeError::NegativeValue)));
        assert!(matches!(Rectangle::new(0.0, 3.0), Err(ShapeError::ZeroValue)));
        assert!(matches!(Rectangle::new(4.0, 0.0), Err(ShapeError::ZeroValue)));
        assert!(matches!(Rectangle::new(f64::NAN, 3.0), Err(ShapeError::InvalidDimension(_))));
        assert!(matches!(Rectangle::new(4.0, f64::INFINITY), Err(ShapeError::InvalidDimension(_))));
    }

    #[test]
    fn rectangle_calculations() {
        let rect = Rectangle::new(5.0, 4.0).unwrap();
        assert_eq!(rect.area(), 20.0);
        assert_eq!(rect.perimeter(), 18.0);
        assert_eq!(rect.name(), "Rectangle");
    }

    #[test]
    fn rectangle_setters() {
        let mut rect = Rectangle::new(4.0, 3.0).unwrap();
        
        assert!(rect.set_width(6.0).is_ok());
        assert_eq!(rect.width(), 6.0);
        
        assert!(rect.set_height(5.0).is_ok());
        assert_eq!(rect.height(), 5.0);
        
        assert!(rect.set_width(-1.0).is_err());
        assert!(rect.set_height(0.0).is_err());
    }

    #[test]
    fn rectangle_scaling() {
        let mut rect = Rectangle::new(4.0, 3.0).unwrap();
        
        assert!(rect.scale(2.0).is_ok());
        assert_eq!(rect.width(), 8.0);
        assert_eq!(rect.height(), 6.0);
        
        assert!(rect.scale(0.5).is_ok());
        assert_eq!(rect.width(), 4.0);
        assert_eq!(rect.height(), 3.0);
        
        assert!(rect.scale(-1.0).is_err());
        assert!(rect.scale(0.0).is_err());
        assert!(rect.scale(f64::NAN).is_err());
    }

    #[test]
    fn circle_creation_valid() {
        let circle = Circle::new(2.5);
        assert!(circle.is_ok());
        let circle = circle.unwrap();
        assert_eq!(circle.radius(), 2.5);
    }

    #[test]
    fn circle_creation_invalid() {
        assert!(matches!(Circle::new(-1.0), Err(ShapeError::NegativeValue)));
        assert!(matches!(Circle::new(0.0), Err(ShapeError::ZeroValue)));
        assert!(matches!(Circle::new(f64::NAN), Err(ShapeError::InvalidDimension(_))));
        assert!(matches!(Circle::new(f64::INFINITY), Err(ShapeError::InvalidDimension(_))));
    }

    #[test]
    fn circle_calculations() {
        let circle = Circle::new(3.0).unwrap();
        let expected_area = std::f64::consts::PI * 9.0;
        let expected_perimeter = 2.0 * std::f64::consts::PI * 3.0;
        
        assert!((circle.area() - expected_area).abs() < 1e-10);
        assert!((circle.perimeter() - expected_perimeter).abs() < 1e-10);
        assert_eq!(circle.name(), "Circle");
    }

    #[test]
    fn circle_setters_and_scaling() {
        let mut circle = Circle::new(2.0).unwrap();
        
        assert!(circle.set_radius(3.0).is_ok());
        assert_eq!(circle.radius(), 3.0);
        
        assert!(circle.scale(2.0).is_ok());
        assert_eq!(circle.radius(), 6.0);
        
        assert!(circle.set_radius(-1.0).is_err());
        assert!(circle.scale(0.0).is_err());
    }

    #[test]
    fn shape_trait_polymorphism() {
        let shapes: Vec<Box<dyn Shape>> = vec![
            Box::new(Rectangle::new(4.0, 3.0).unwrap()),
            Box::new(Circle::new(2.0).unwrap()),
        ];
        
        assert_eq!(shapes[0].area(), 12.0);
        assert_eq!(shapes[0].name(), "Rectangle");
        
        let circle_area = std::f64::consts::PI * 4.0;
        assert!((shapes[1].area() - circle_area).abs() < 1e-10);
        assert_eq!(shapes[1].name(), "Circle");
    }
}

// ===== CALCULATOR TESTS =====

#[cfg(test)]
mod calculator_tests {
    use super::*;

    #[test]
    fn calculator_creation() {
        let calc = Calculator::new();
        assert_eq!(calc.current_value(), 0);
        
        let calc_with_cap = Calculator::with_capacity(50);
        assert_eq!(calc_with_cap.current_value(), 0);
    }

    #[test]
    fn basic_arithmetic() {
        let mut calc = Calculator::new();
        
        assert_eq!(calc.add(10).unwrap(), 10);
        assert_eq!(calc.subtract(3).unwrap(), 7);
        assert_eq!(calc.multiply(4).unwrap(), 28);
        assert_eq!(calc.divide(7).unwrap(), 4);
        assert_eq!(calc.modulo(3).unwrap(), 1);
    }

    #[test]
    fn advanced_operations() {
        let mut calc = Calculator::new();
        
        calc.add(5).unwrap();
        assert_eq!(calc.negate().unwrap(), -5);
        assert_eq!(calc.negate().unwrap(), 5);
        
        calc.clear();
        calc.add(3).unwrap();
        assert_eq!(calc.power(4).unwrap(), 81);
        
        calc.clear();
        calc.add(5).unwrap();
        assert_eq!(calc.factorial().unwrap(), 120);
    }

    #[test]
    fn error_handling() {
        let mut calc = Calculator::new();
        
        // Division by zero
        assert!(matches!(calc.divide(0), Err(CalculatorError::DivisionByZero)));
        assert!(matches!(calc.modulo(0), Err(CalculatorError::DivisionByZero)));
        
        // Overflow
        calc.add(i64::MAX).unwrap();
        assert!(matches!(calc.add(1), Err(CalculatorError::Overflow)));
        
        calc.clear();
        calc.add(i64::MIN).unwrap();
        assert!(matches!(calc.negate(), Err(CalculatorError::Overflow)));
        
        // Invalid factorial
        calc.clear();
        calc.subtract(1).unwrap();
        assert!(matches!(calc.factorial(), Err(CalculatorError::Invalid(_))));
        
        calc.clear();
        calc.add(25).unwrap();
        assert!(matches!(calc.factorial(), Err(CalculatorError::Overflow)));
    }

    #[test]
    fn history_functionality() {
        let mut calc = Calculator::new();
        
        calc.add(10).unwrap();
        calc.multiply(2).unwrap();
        calc.subtract(5).unwrap();
        
        assert_eq!(calc.history().len(), 3);
        assert_eq!(calc.current_value(), 15);
        
        let history_string = calc.history_as_string();
        assert!(history_string.contains("0 + 10 = 10"));
        assert!(history_string.contains("10 × 2 = 20"));
        assert!(history_string.contains("20 - 5 = 15"));
    }

    #[test]
    fn history_capacity() {
        let mut calc = Calculator::with_capacity(2);
        
        calc.add(1).unwrap();
        calc.add(1).unwrap();
        calc.add(1).unwrap(); // This should push out the first entry
        
        assert_eq!(calc.history().len(), 2);
        assert!(!calc.history_as_string().contains("0 + 1 = 1"));
    }

    #[test]
    fn clear_operations() {
        let mut calc = Calculator::new();
        
        calc.add(10).unwrap();
        calc.multiply(2).unwrap();
        
        calc.clear_history();
        assert_eq!(calc.history().len(), 0);
        assert_eq!(calc.current_value(), 20);
        
        calc.clear();
        assert_eq!(calc.current_value(), 0);
        assert_eq!(calc.history().len(), 0);
    }

    #[test]
    fn standalone_functions() {
        use crate::calculator::{checked_add, checked_subtract, checked_multiply, checked_divide};
        
        assert_eq!(checked_add(5, 3).unwrap(), 8);
        assert_eq!(checked_subtract(10, 4).unwrap(), 6);
        assert_eq!(checked_multiply(7, 6).unwrap(), 42);
        assert_eq!(checked_divide(15, 3).unwrap(), 5);
        
        assert!(checked_add(i64::MAX, 1).is_err());
        assert!(checked_divide(10, 0).is_err());
    }

    #[test]
    fn saturating_operations() {
        use crate::calculator::saturating;
        
        assert_eq!(saturating::add(i64::MAX, 1), i64::MAX);
        assert_eq!(saturating::subtract(i64::MIN, 1), i64::MIN);
        assert_eq!(saturating::multiply(i64::MAX, 2), i64::MAX);
    }

    #[test]
    fn wrapping_operations() {
        use crate::calculator::wrapping;
        
        assert_eq!(wrapping::add(i64::MAX, 1), i64::MIN);
        assert_eq!(wrapping::subtract(i64::MIN, 1), i64::MAX);
        assert_eq!(wrapping::multiply(-1, i64::MIN), i64::MIN);
    }
}

// ===== INTEGRATION TESTS =====

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn calculator_with_shape_dimensions() {
        let mut calc = Calculator::new();
        
        // Calculate rectangle area using calculator
        calc.add(5).unwrap();  // width
        calc.multiply(3).unwrap();  // height
        let calc_area = calc.current_value();
        
        // Compare with rectangle area
        let rect = Rectangle::new(5.0, 3.0).unwrap();
        assert_eq!(calc_area as f64, rect.area());
    }

    #[test]
    fn error_display_formatting() {
        // Test error display implementations
        let shape_err = ShapeError::NegativeValue;
        assert_eq!(format!("{}", shape_err), "negative values are not allowed");
        
        let calc_err = CalculatorError::DivisionByZero;
        assert_eq!(format!("{}", calc_err), "division by zero");
    }
}
    