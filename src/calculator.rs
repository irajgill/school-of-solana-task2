use std::fmt;

// Calculator error types
#[derive(Debug, PartialEq)]
pub enum CalculatorError {
    DivisionByZero,
    Overflow,
    Underflow,
    InvalidOperation(String),
    HistoryEmpty,
}

impl fmt::Display for CalculatorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CalculatorError::DivisionByZero => write!(f, "Division by zero is not allowed"),
            CalculatorError::Overflow => write!(f, "Arithmetic overflow occurred"),
            CalculatorError::Underflow => write!(f, "Arithmetic underflow occurred"),
            CalculatorError::InvalidOperation(msg) => write!(f, "Invalid operation: {}", msg),
            CalculatorError::HistoryEmpty => write!(f, "Calculator history is empty"),
        }
    }
}

impl std::error::Error for CalculatorError {}

// Operation enum
#[derive(Debug, Clone, PartialEq)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
    Modulo,
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Operation::Add => write!(f, "+"),
            Operation::Subtract => write!(f, "-"),
            Operation::Multiply => write!(f, "*"),
            Operation::Divide => write!(f, "/"),
            Operation::Power => write!(f, "^"),
            Operation::Modulo => write!(f, "%"),
        }
    }
}

// History entry to track calculations
#[derive(Debug, Clone)]
pub struct HistoryEntry {
    pub operand1: f64,
    pub operation: Operation,
    pub operand2: f64,
    pub result: f64,
    pub timestamp: std::time::SystemTime,
}

impl fmt::Display for HistoryEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {} = {}", 
               self.operand1, self.operation, self.operand2, self.result)
    }
}

// Main Calculator struct
#[derive(Debug)]
pub struct Calculator {
    current_value: f64,
    history: Vec<HistoryEntry>,
    max_history_size: usize,
}

impl Calculator {
    // Constructor
    pub fn new() -> Self {
        Calculator {
            current_value: 0.0,
            history: Vec::new(),
            max_history_size: 100, // Limit history to prevent memory issues
        }
    }
    
    // Constructor with custom history size
    pub fn with_history_size(max_size: usize) -> Self {
        Calculator {
            current_value: 0.0,
            history: Vec::new(),
            max_history_size: max_size,
        }
    }
    
    // Get current value
    pub fn current_value(&self) -> f64 {
        self.current_value
    }
    
    // Set current value
    pub fn set_value(&mut self, value: f64) {
        self.current_value = value;
    }
    
    // Clear calculator (reset to 0)
    pub fn clear(&mut self) {
        self.current_value = 0.0;
    }
    
    // Clear history
    pub fn clear_history(&mut self) {
        self.history.clear();
    }
    
    // Add two numbers
    pub fn add(&mut self, operand: f64) -> Result<f64, CalculatorError> {
        let result = self.check_overflow_add(self.current_value, operand)?;
        self.record_operation(Operation::Add, operand, result);
        self.current_value = result;
        Ok(result)
    }
    
    // Subtract two numbers
    pub fn subtract(&mut self, operand: f64) -> Result<f64, CalculatorError> {
        let result = self.check_overflow_sub(self.current_value, operand)?;
        self.record_operation(Operation::Subtract, operand, result);
        self.current_value = result;
        Ok(result)
    }
    
    // Multiply two numbers
    pub fn multiply(&mut self, operand: f64) -> Result<f64, CalculatorError> {
        let result = self.check_overflow_mul(self.current_value, operand)?;
        self.record_operation(Operation::Multiply, operand, result);
        self.current_value = result;
        Ok(result)
    }
    
    // Divide two numbers
    pub fn divide(&mut self, operand: f64) -> Result<f64, CalculatorError> {
        if operand == 0.0 {
            return Err(CalculatorError::DivisionByZero);
        }
        
        let result = self.current_value / operand;
        if !result.is_finite() {
            return Err(CalculatorError::Overflow);
        }
        
        self.record_operation(Operation::Divide, operand, result);
        self.current_value = result;
        Ok(result)
    }
    
    // Power operation
    pub fn power(&mut self, exponent: f64) -> Result<f64, CalculatorError> {
        let result = self.current_value.powf(exponent);
        
        if !result.is_finite() {
            return Err(CalculatorError::Overflow);
        }
        
        self.record_operation(Operation::Power, exponent, result);
        self.current_value = result;
        Ok(result)
    }
    
    // Modulo operation
    pub fn modulo(&mut self, operand: f64) -> Result<f64, CalculatorError> {
        if operand == 0.0 {
            return Err(CalculatorError::DivisionByZero);
        }
        
        let result = self.current_value % operand;
        self.record_operation(Operation::Modulo, operand, result);
        self.current_value = result;
        Ok(result)
    }
    
    // Perform operation based on Operation enum
    pub fn perform_operation(&mut self, operation: Operation, operand: f64) -> Result<f64, CalculatorError> {
        match operation {
            Operation::Add => self.add(operand),
            Operation::Subtract => self.subtract(operand),
            Operation::Multiply => self.multiply(operand),
            Operation::Divide => self.divide(operand),
            Operation::Power => self.power(operand),
            Operation::Modulo => self.modulo(operand),
        }
    }
    
    // Get calculation history
    pub fn get_history(&self) -> &[HistoryEntry] {
        &self.history
    }
    
    // Get last calculation result
    pub fn last_result(&self) -> Result<f64, CalculatorError> {
        match self.history.last() {
            Some(entry) => Ok(entry.result),
            None => Err(CalculatorError::HistoryEmpty),
        }
    }
    
    // Get history size
    pub fn history_size(&self) -> usize {
        self.history.len()
    }
    
    // Undo last operation
    pub fn undo(&mut self) -> Result<f64, CalculatorError> {
        match self.history.pop() {
            Some(entry) => {
                self.current_value = entry.operand1;
                Ok(self.current_value)
            }
            None => Err(CalculatorError::HistoryEmpty),
        }
    }
    
    // Private helper methods
    
    // Record operation in history
    fn record_operation(&mut self, operation: Operation, operand: f64, result: f64) {
        let entry = HistoryEntry {
            operand1: self.current_value,
            operation,
            operand2: operand,
            result,
            timestamp: std::time::SystemTime::now(),
        };
        
        self.history.push(entry);
        
        // Limit history size
        if self.history.len() > self.max_history_size {
            self.history.remove(0);
        }
    }
    
    // Check for addition overflow
    fn check_overflow_add(&self, a: f64, b: f64) -> Result<f64, CalculatorError> {
        let result = a + b;
        if !result.is_finite() {
            if result.is_infinite() {
                Err(CalculatorError::Overflow)
            } else {
                Err(CalculatorError::InvalidOperation("NaN result".to_string()))
            }
        } else {
            Ok(result)
        }
    }
    
    // Check for subtraction overflow/underflow
    fn check_overflow_sub(&self, a: f64, b: f64) -> Result<f64, CalculatorError> {
        let result = a - b;
        if !result.is_finite() {
            if result.is_infinite() {
                if result.is_sign_positive() {
                    Err(CalculatorError::Overflow)
                } else {
                    Err(CalculatorError::Underflow)
                }
            } else {
                Err(CalculatorError::InvalidOperation("NaN result".to_string()))
            }
        } else {
            Ok(result)
        }
    }
    
    // Check for multiplication overflow
    fn check_overflow_mul(&self, a: f64, b: f64) -> Result<f64, CalculatorError> {
        let result = a * b;
        if !result.is_finite() {
            if result.is_infinite() {
                Err(CalculatorError::Overflow)
            } else {
                Err(CalculatorError::InvalidOperation("NaN result".to_string()))
            }
        } else {
            Ok(result)
        }
    }
}

impl Default for Calculator {
    fn default() -> Self {
        Self::new()
    }
}

// Utility functions for calculator operations
pub fn calculate_simple(a: f64, operation: Operation, b: f64) -> Result<f64, CalculatorError> {
    match operation {
        Operation::Add => {
            let result = a + b;
            if result.is_finite() { Ok(result) } else { Err(CalculatorError::Overflow) }
        }
        Operation::Subtract => {
            let result = a - b;
            if result.is_finite() { Ok(result) } else { Err(CalculatorError::Underflow) }
        }
        Operation::Multiply => {
            let result = a * b;
            if result.is_finite() { Ok(result) } else { Err(CalculatorError::Overflow) }
        }
        Operation::Divide => {
            if b == 0.0 {
                Err(CalculatorError::DivisionByZero)
            } else {
                let result = a / b;
                if result.is_finite() { Ok(result) } else { Err(CalculatorError::Overflow) }
            }
        }
        Operation::Power => {
            let result = a.powf(b);
            if result.is_finite() { Ok(result) } else { Err(CalculatorError::Overflow) }
        }
        Operation::Modulo => {
            if b == 0.0 {
                Err(CalculatorError::DivisionByZero)
            } else {
                Ok(a % b)
            }
        }
    }
}
