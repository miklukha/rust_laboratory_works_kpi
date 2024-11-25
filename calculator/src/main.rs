use slint::SharedString;
use std::cell::RefCell;
use std::rc::Rc;

slint::include_modules!();

#[derive(Debug)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    None,
}

#[derive(Debug, Clone)]
struct MemoryEntry {
    expression: String,
}

#[derive(Debug)]
struct Calculator {
    current_number: String,
    stored_number: f64,
    operation: Operation,
    new_number: bool,
    memory_history: Vec<MemoryEntry>,
    current_expression: String,
}

impl Calculator {
    fn new() -> Self {
        Calculator {
            current_number: String::from("0"),
            stored_number: 0.0,
            operation: Operation::None,
            new_number: true,
            memory_history: Vec::new(),
            current_expression: String::from("0"),
        }
    }

    fn round_to_decimals(value: f64, decimals: u32) -> f64 {
        let multiplier = 10_f64.powi(decimals as i32);
        (value * multiplier).round() / multiplier
    }

    fn add_digit(&mut self, digit: &str) {
        if self.new_number {
            self.current_number = digit.to_string();
            self.current_expression = digit.to_string();
            self.new_number = false;
        } else {
            self.current_number.push_str(digit);
            self.current_expression.push_str(digit);
        }
    }

    fn calculate(&mut self) -> f64 {
        let current = self.current_number.parse::<f64>().unwrap_or(0.0);
        let result = match self.operation {
            Operation::Add => self.stored_number + current,
            Operation::Subtract => self.stored_number - current,
            Operation::Multiply => self.stored_number * current,
            Operation::Divide => {
                if current != 0.0 {
                    Self::round_to_decimals(self.stored_number / current, 4)
                } else {
                    f64::NAN
                }
            }
            Operation::None => current,
        };

        let full_expression = format!(
            "{} {} {} = {}",
            self.stored_number,
            match self.operation {
                Operation::Add => "+",
                Operation::Subtract => "-",
                Operation::Multiply => "*",
                Operation::Divide => "/",
                Operation::None => "",
            },
            self.current_number,
            result
        );
        self.current_expression = full_expression.clone();

        result
    }

    fn format_memory_history(&self) -> String {
        self.memory_history
            .iter()
            .enumerate()
            .map(|(i, entry)| format!("{}. {}", i + 1, entry.expression))
            .collect::<Vec<String>>()
            .join("\n")
    }
}

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let calc = Rc::new(RefCell::new(Calculator::new()));

    let ui_handle = ui.as_weak();
    let calc_clone = calc.clone();
    ui.on_number_clicked(move |number: SharedString| {
        let ui = ui_handle.unwrap();
        calc_clone.borrow_mut().add_digit(&number.to_string());
        ui.set_result(calc_clone.borrow().current_number.clone().into());
    });

    let ui_handle = ui.as_weak();
    let calc_clone = calc.clone();
    ui.on_operation_clicked(move |op: SharedString| {
        let ui = ui_handle.unwrap();
        let mut calc = calc_clone.borrow_mut();
        let result = calc.calculate();
        calc.stored_number = result;
        calc.new_number = true;
        calc.operation = match op.as_str() {
            "+" => Operation::Add,
            "-" => Operation::Subtract,
            "*" => Operation::Multiply,
            "/" => Operation::Divide,
            _ => Operation::None,
        };
        ui.set_result(result.to_string().into());
    });

    let ui_handle = ui.as_weak();
    let calc_clone = calc.clone();
    ui.on_command_clicked(move |cmd: SharedString| {
        let ui = ui_handle.unwrap();
        match cmd.as_str() {
            "clear" => {
                let mut calc = calc_clone.borrow_mut();
                calc.current_number = String::from("0");
                calc.stored_number = 0.0;
                calc.operation = Operation::None;
                calc.new_number = true;
                ui.set_result(SharedString::from("0"));
            }
            "save" => {
                let mut calc = calc_clone.borrow_mut();

                let current_expression = calc.current_expression.clone();

                calc.memory_history.push(MemoryEntry {
                    expression: current_expression,
                });

                ui.set_memory(calc.format_memory_history().into());
            }
            "equals" => {
                let mut calc = calc_clone.borrow_mut();
                let result = calc.calculate();
                calc.stored_number = 0.0;
                calc.operation = Operation::None;
                calc.current_number = result.to_string();
                calc.new_number = true;
                ui.set_result(result.to_string().into());
            }
            _ => {}
        }
    });

    ui.run()
}
