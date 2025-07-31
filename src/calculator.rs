///-------------------------------------------------------------------------------
///
/// This is your calculator implementation task 
/// to practice enums, structs, and methods.
/// 
/// Complete the implementation of the Calculator struct and its methods.
/// 
/// The calculator should support basic arithmetic 
/// operations (addition, subtraction, multiplication)
/// with overflow protection and maintain a history 
/// of operations.
/// 
/// Tasks:
/// 1. Implement the OperationType enum methods
/// 2. Implement the Operation struct constructor
/// 3. Implement all Calculator methods
/// 
///-------------------------------------------------------------------------------

#[derive(Clone)]
pub enum OperationType {
    Addition,
    Subtraction,
    Multiplication
}

impl OperationType {
    // Return the string representation of the operation sign
    pub fn get_sign(&self) -> &str {
        match self {
            OperationType::Addition => "+",
            OperationType::Subtraction => "-",
            OperationType::Multiplication => "*",
        }
    }
    
    // Perform the operation on two i64 numbers with overflow protection
    pub fn perform(&self, x: i64, y: i64) -> Option<i64> {
        match self {
            OperationType::Addition => x.checked_add(y),
            OperationType::Subtraction => x.checked_sub(y),
            OperationType::Multiplication => x.checked_mul(y),
        }
    }
}

#[derive(Clone)]
pub struct Operation {
    pub first_num: i64,
    pub second_num: i64,
    pub operation_type: OperationType
}

impl Operation {
    // Create a new Operation with the given parameters
    pub fn new(first_num: i64, second_num: i64, operation_type: OperationType) -> Self {
        Operation {
            first_num,
            second_num,
            operation_type,
        }
    }
}

pub struct Calculator {
    pub history: Vec<Operation>
}

impl Calculator {
    // Create a new Calculator with empty history
    pub fn new() -> Self {
        Calculator {
            history: Vec::new(),
        }
    }
    
    // Perform addition and store successful operations in history
    pub fn addition(&mut self, x: i64, y: i64) -> Option<i64> {
        let result = OperationType::Addition.perform(x, y);
        if result.is_some() {
            let operation = Operation::new(x, y, OperationType::Addition);
            self.history.push(operation);
        }
        result
    }
    
    // Perform subtraction and store successful operations in history
    pub fn subtraction(&mut self, x: i64, y: i64) -> Option<i64> {
        let result = OperationType::Subtraction.perform(x, y);
        if result.is_some() {
            let operation = Operation::new(x, y, OperationType::Subtraction);
            self.history.push(operation);
        }
        result
    }
    
    // Perform multiplication and store successful operations in history
    pub fn multiplication(&mut self, x: i64, y: i64) -> Option<i64> {
        let result = OperationType::Multiplication.perform(x, y);
        if result.is_some() {
            let operation = Operation::new(x, y, OperationType::Multiplication);
            self.history.push(operation);
        }
        result
    }
    
    // Generate a formatted string showing all operations in history
    pub fn show_history(&self) -> String {
        let mut result = String::new();
        for (index, operation) in self.history.iter().enumerate() {
            let op_result = operation.operation_type.perform(operation.first_num, operation.second_num);
            if let Some(value) = op_result {
                result.push_str(&format!("{}: {} {} {} = {}\n", 
                    index, 
                    operation.first_num, 
                    operation.operation_type.get_sign(), 
                    operation.second_num, 
                    value
                ));
            }
        }
        result
    }
    
    // Repeat an operation from history by index
    pub fn repeat(&mut self, operation_index: usize) -> Option<i64> {
        if operation_index >= self.history.len() {
            return None;
        }
        
        let operation = self.history[operation_index].clone();
        match operation.operation_type {
            OperationType::Addition => self.addition(operation.first_num, operation.second_num),
            OperationType::Subtraction => self.subtraction(operation.first_num, operation.second_num),
            OperationType::Multiplication => self.multiplication(operation.first_num, operation.second_num),
        }
    }
    
    // Clear all operations from history
    pub fn clear_history(&mut self) {
        self.history.clear();
    }
}
