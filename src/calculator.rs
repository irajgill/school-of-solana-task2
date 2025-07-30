
use core::fmt;
use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq)]
pub enum CalculatorError {
    Overflow,
    Underflow,
    DivisionByZero,
    Invalid(String),
}
impl fmt::Display for CalculatorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use CalculatorError::*;
        match self {
            Overflow           => write!(f, "arithmetic overflow"),
            Underflow          => write!(f, "arithmetic underflow"),
            DivisionByZero     => write!(f, "division by zero"),
            Invalid(msg)       => write!(f, "invalid op: {msg}"),
        }
    }
}
impl std::error::Error for CalculatorError {}

#[derive(Debug, Clone)]
pub struct HistoryEntry { pub op: String, pub result: i64 }
impl fmt::Display for HistoryEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} = {}", self.op, self.result)
    }
}

// Calculator

pub struct Calculator {
    current:  i64,
    history:  VecDeque<HistoryEntry>,
    capacity: usize,
}
impl Calculator {
    pub fn new() -> Self { Self::with_capacity(100) }
    pub fn with_capacity(cap: usize) -> Self {
        Self { current: 0, history: VecDeque::new(), capacity: cap }
    }

    pub fn current_value(&self) -> i64 { self.current }

    pub fn clear(&mut self)  { self.current = 0; self.history.clear(); }
    pub fn clear_history(&mut self) { self.history.clear(); }

    pub fn history(&self) -> &VecDeque<HistoryEntry> { &self.history }
    pub fn history_as_string(&self) -> String {
        self.history.iter()
            .enumerate()
            .map(|(i,e)| format!("{:>3}. {e}", i+1))
            .collect::<Vec<_>>()
            .join("\n")
    }

    // Operations

    pub fn add(&mut self, value: i64) -> Result<i64, CalculatorError> {
        self.apply(" + ", value, i64::checked_add, CalculatorError::Overflow)
    }
    pub fn subtract(&mut self, value: i64) -> Result<i64, CalculatorError> {
        self.apply(" - ", value, i64::checked_sub, CalculatorError::Underflow)
    }
    pub fn multiply(&mut self, value: i64) -> Result<i64, CalculatorError> {
        self.apply(" × ", value, i64::checked_mul, CalculatorError::Overflow)
    }
    pub fn divide(&mut self, value: i64) -> Result<i64, CalculatorError> {
        if value == 0 { return Err(CalculatorError::DivisionByZero); }
        self.apply(" ÷ ", value, i64::checked_div, CalculatorError::Overflow)
    }
    pub fn modulo(&mut self, value: i64) -> Result<i64, CalculatorError> {
        if value == 0 { return Err(CalculatorError::DivisionByZero); }
        self.apply(" % ", value, i64::checked_rem, CalculatorError::Overflow)
    }
    pub fn negate(&mut self) -> Result<i64, CalculatorError> {
        let new = self.current.checked_neg().ok_or(CalculatorError::Overflow)?;
        self.push_history(format!("neg({})", self.current), new);
        self.current = new;
        Ok(new)
    }
    pub fn power(&mut self, exp: u32) -> Result<i64, CalculatorError> {
        let new = self.current.checked_pow(exp).ok_or(CalculatorError::Overflow)?;
        self.push_history(format!("pow({}, {exp})", self.current), new);
        self.current = new;
        Ok(new)
    }
    pub fn factorial(&mut self) -> Result<i64, CalculatorError> {
        if self.current < 0 { return Err(CalculatorError::Invalid("factorial of negative".into())); }
        if self.current > 20 { return Err(CalculatorError::Overflow); } // 21! > i64::MAX
        let mut acc: i64 = 1;
        for n in 1..=self.current {
            acc = acc.checked_mul(n).ok_or(CalculatorError::Overflow)?;
        }
        self.push_history(format!("{}!", self.current), acc);
        self.current = acc;
        Ok(acc)
    }

    // Helper function to apply an operation and handle history

    fn apply<F>(
        &mut self,
        sym: &str,
        val: i64,
        op: F,
        err: CalculatorError,
    ) -> Result<i64, CalculatorError>
    where
        F: FnOnce(i64, i64) -> Option<i64>,
    {
        let new = op(self.current, val).ok_or(err)?;
        self.push_history(format!("{}{}{}", self.current, sym, val), new);
        self.current = new;
        Ok(new)
    }

    fn push_history(&mut self, op: String, result: i64) {
        if self.history.len() == self.capacity { self.history.pop_front(); }
        self.history.push_back(HistoryEntry { op, result });
    }
}

// Stand-alone checked helpers

pub fn checked_add(a: i64, b: i64) -> Result<i64, CalculatorError> {
    a.checked_add(b).ok_or(CalculatorError::Overflow)
}
pub fn checked_subtract(a: i64, b: i64) -> Result<i64, CalculatorError> {
    a.checked_sub(b).ok_or(CalculatorError::Underflow)
}
pub fn checked_multiply(a: i64, b: i64) -> Result<i64, CalculatorError> {
    a.checked_mul(b).ok_or(CalculatorError::Overflow)
}
pub fn checked_divide(a: i64, b: i64) -> Result<i64, CalculatorError> {
    if b == 0 { return Err(CalculatorError::DivisionByZero); }
    a.checked_div(b).ok_or(CalculatorError::Overflow)
}

// Saturating & wrapping modules

pub mod saturating {
    pub fn add(a: i64, b: i64)      -> i64 { a.saturating_add(b) }
    pub fn subtract(a: i64, b: i64) -> i64 { a.saturating_sub(b) }
    pub fn multiply(a: i64, b: i64) -> i64 { a.saturating_mul(b) }
}
pub mod wrapping {
    pub fn add(a: i64, b: i64)      -> i64 { a.wrapping_add(b) }
    pub fn subtract(a: i64, b: i64) -> i64 { a.wrapping_sub(b) }
    pub fn multiply(a: i64, b: i64) -> i64 { a.wrapping_mul(b) }
}
