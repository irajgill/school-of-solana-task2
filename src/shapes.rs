use std::fmt;

// Custom error type for shape validation
#[derive(Debug, PartialEq)]
pub enum ShapeError {
    InvalidDimension(String),
    NegativeValue(String),
    ZeroValue(String),
}

impl fmt::Display for ShapeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ShapeError::InvalidDimension(msg) => write!(f, "Invalid dimension: {}", msg),
            ShapeError::NegativeValue(msg) => write!(f, "Negative value not allowed: {}", msg),
            ShapeError::ZeroValue(msg) => write!(f, "Zero value not allowed: {}", msg),
        }
    }
}

impl std::error::Error for ShapeError {}

// Shape trait that all shapes must implement
pub trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
    fn name(&self) -> &'static str;
    
    // Default implementation for displaying shape info
    fn display_info(&self) {
        println!("{}: Area = {:.2}, Perimeter = {:.2}", 
                 self.name(), self.area(), self.perimeter());
    }
}

// Rectangle struct
#[derive(Debug, Clone)]
pub struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    // Constructor with validation
    pub fn new(width: f64, height: f64) -> Result<Self, ShapeError> {
        if width < 0.0 {
            return Err(ShapeError::NegativeValue("Width cannot be negative".to_string()));
        }
        if height < 0.0 {
            return Err(ShapeError::NegativeValue("Height cannot be negative".to_string()));
        }
        if width == 0.0 {
            return Err(ShapeError::ZeroValue("Width cannot be zero".to_string()));
        }
        if height == 0.0 {
            return Err(ShapeError::ZeroValue("Height cannot be zero".to_string()));
        }
        
        Ok(Rectangle { width, height })
    }
    
    pub fn width(&self) -> f64 {
        self.width
    }
    
    pub fn height(&self) -> f64 {
        self.height
    }
    
    // Check if rectangle is a square
    pub fn is_square(&self) -> bool {
        (self.width - self.height).abs() < f64::EPSILON
    }
    
    // Scale the rectangle by a factor
    pub fn scale(&mut self, factor: f64) -> Result<(), ShapeError> {
        if factor <= 0.0 {
            return Err(ShapeError::InvalidDimension("Scale factor must be positive".to_string()));
        }
        
        self.width *= factor;
        self.height *= factor;
        Ok(())
    }
}

// Implement Shape trait for Rectangle
impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
    
    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
    
    fn name(&self) -> &'static str {
        "Rectangle"
    }
}

// Circle struct
#[derive(Debug, Clone)]
pub struct Circle {
    radius: f64,
}

impl Circle {
    // Constructor with validation
    pub fn new(radius: f64) -> Result<Self, ShapeError> {
        if radius < 0.0 {
            return Err(ShapeError::NegativeValue("Radius cannot be negative".to_string()));
        }
        if radius == 0.0 {
            return Err(ShapeError::ZeroValue("Radius cannot be zero".to_string()));
        }
        
        Ok(Circle { radius })
    }
    
    pub fn radius(&self) -> f64 {
        self.radius
    }
    
    pub fn diameter(&self) -> f64 {
        2.0 * self.radius
    }
    
    // Scale the circle by a factor
    pub fn scale(&mut self, factor: f64) -> Result<(), ShapeError> {
        if factor <= 0.0 {
            return Err(ShapeError::InvalidDimension("Scale factor must be positive".to_string()));
        }
        
        self.radius *= factor;
        Ok(())
    }
}

// Implement Shape trait for Circle
impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
    
    fn perimeter(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }
    
    fn name(&self) -> &'static str {
        "Circle"
    }
}

// Compare two shapes by area using trait objects (no generics)
pub fn compare_areas(shape1: &dyn Shape, shape2: &dyn Shape) -> std::cmp::Ordering {
    shape1.area()
        .partial_cmp(&shape2.area())
        .unwrap_or(std::cmp::Ordering::Equal)
}

// Find the largest shape from a collection of boxed trait objects
pub fn find_largest_shape<'a>(
    shapes: &'a [Box<dyn Shape>],
) -> Option<&'a Box<dyn Shape>> {
    shapes
        .iter()
        .max_by(|a, b| compare_areas(&**a, &**b))
}
