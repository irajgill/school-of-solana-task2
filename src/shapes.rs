use core::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum ShapeError {
    NegativeValue,
    ZeroValue,
    InvalidDimension(String),
}

impl fmt::Display for ShapeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ShapeError::*;
        match self {
            NegativeValue          => write!(f, "negative values are not allowed"),
            ZeroValue              => write!(f, "zero values are not allowed"),
            InvalidDimension(msg)  => write!(f, "invalid dimension: {msg}"),
        }
    }
}
impl std::error::Error for ShapeError {}

/// Common behaviour for every 2-D shape
pub trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
    fn name(&self) -> &'static str;
}

/// Rectangle

#[derive(Debug, Clone, PartialEq)]
pub struct Rectangle {
    width:  f64,
    height: f64,
}

impl Rectangle {
    pub fn new(width: f64, height: f64) -> Result<Self, ShapeError> {
        if width  < 0.0 || height < 0.0 { return Err(ShapeError::NegativeValue); }
        if width == 0.0 || height == 0.0 { return Err(ShapeError::ZeroValue); }
        if width.is_nan()  || height.is_nan() ||
           width.is_infinite() || height.is_infinite()
        {
            return Err(ShapeError::InvalidDimension("NaN or ∞".into()));
        }
        Ok(Self { width, height })
    }

        /// Getters for width and height
    pub fn width (&self) -> f64 { self.width  }
    pub fn height(&self) -> f64 { self.height }

    pub fn set_width (&mut self, w: f64) -> Result<(), ShapeError> { *self = Self::new(w, self.height )?; Ok(()) }
    pub fn set_height(&mut self, h: f64) -> Result<(), ShapeError> { *self = Self::new(self.width, h)?; Ok(()) }

    pub fn scale(&mut self, factor: f64) -> Result<(), ShapeError> {
        if factor <= 0.0 || factor.is_nan() || factor.is_infinite() {
            return Err(ShapeError::InvalidDimension("scale factor".into()));
        }
        self.width  *= factor;
        self.height *= factor;
        Ok(())
    }
}

/// Rectangle implements Shape
impl Shape for Rectangle {
    fn area(&self)      -> f64 { self.width * self.height }
    fn perimeter(&self) -> f64 { 2.0 * (self.width + self.height) }
    fn name(&self)      -> &'static str { "Rectangle" }
}
impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Rectangle({}×{})", self.width, self.height)
    }
}

// Circle

#[derive(Debug, Clone, PartialEq)]
pub struct Circle {
    radius: f64,
}

impl Circle {
    pub fn new(radius: f64) -> Result<Self, ShapeError> {
        if radius < 0.0      { return Err(ShapeError::NegativeValue); }
        if radius == 0.0     { return Err(ShapeError::ZeroValue); }
        if radius.is_nan() || radius.is_infinite() {
            return Err(ShapeError::InvalidDimension("NaN or ∞".into()));
        }
        Ok(Self { radius })
    }

    pub fn radius(&self) -> f64 { self.radius }

    pub fn set_radius(&mut self, r: f64) -> Result<(), ShapeError> {
        *self = Self::new(r)?;
        Ok(())
    }
    pub fn scale(&mut self, factor: f64) -> Result<(), ShapeError> {
        if factor <= 0.0 || factor.is_nan() || factor.is_infinite() {
            return Err(ShapeError::InvalidDimension("scale factor".into()));
        }
        self.radius *= factor;
        Ok(())
    }
}

/// Circle implements Shape
impl Shape for Circle {
    fn area(&self)      -> f64 { core::f64::consts::PI * self.radius * self.radius }
    fn perimeter(&self) -> f64 { 2.0 * core::f64::consts::PI * self.radius }
    fn name(&self)      -> &'static str { "Circle" }
}
impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Circle(r={})", self.radius)
    }
}
