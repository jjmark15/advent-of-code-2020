use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;

enum BootOperation {
    Jump,
    Accumulate,
    NoOperation,
}

impl FromStr for BootOperation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "nop" => Ok(BootOperation::NoOperation),
            "acc" => Ok(BootOperation::Accumulate),
            "jmp" => Ok(BootOperation::Jump),
            _ => Err(anyhow::Error::msg(format!(
                "Could not parse boot operation from '{}'",
                s
            ))),
        }
    }
}

struct BootInstruction {
    operation: BootOperation,
    argument: i64,
}

impl BootInstruction {
    fn new(operation: BootOperation, argument: i64) -> Self {
        BootInstruction {
            operation,
            argument,
        }
    }

    fn operation(&self) -> &BootOperation {
        &self.operation
    }

    fn argument(&self) -> i64 {
        self.argument
    }
}

impl FromStr for BootInstruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^(?P<operation>\w{3}) (?P<value>[\+-]\d+)$").unwrap();
        }

        let operation: BootOperation = RE
            .captures(s)
            .ok_or_else(|| anyhow::Error::msg("Could not parse boot instruction operation"))?
            .name("operation")
            .unwrap()
            .as_str()
            .parse()?;

        let value: i64 = RE
            .captures(s)
            .ok_or_else(|| anyhow::Error::msg("Could not parse boot instruction argument"))?
            .name("value")
            .unwrap()
            .as_str()
            .parse()?;

        Ok(BootInstruction::new(operation, value))
    }
}

struct Executor {
    accumulated_value: i64,
    boot_instructions: Vec<BootInstruction>,
    next_instruction_index: usize,
}

impl Executor {
    fn new(boot_instructions: Vec<BootInstruction>) -> Self {
        Executor {
            accumulated_value: 0,
            boot_instructions,
            next_instruction_index: 0,
        }
    }

    fn apply_next_instruction(&mut self) -> anyhow::Result<()> {
        let next_instruction = self
            .boot_instructions
            .get(self.next_instruction_index)
            .ok_or_else(|| anyhow::Error::msg("Attempted to access instruction out of range"))?;

        match next_instruction.operation() {
            BootOperation::Jump => {
                let new_next_index =
                    self.next_instruction_index as i64 + next_instruction.argument();
                self.next_instruction_index = new_next_index as usize;
            }
            BootOperation::Accumulate => {
                self.accumulated_value += next_instruction.argument();
                self.next_instruction_index += 1;
            }
            BootOperation::NoOperation => self.next_instruction_index += 1,
        };

        Ok(())
    }

    fn accumulated_value(&self) -> i64 {
        self.accumulated_value
    }

    fn next_instruction_index(&self) -> usize {
        self.next_instruction_index
    }
}

struct ExecutionHistory {
    index_history: Vec<usize>,
}

impl ExecutionHistory {
    fn new() -> Self {
        ExecutionHistory {
            index_history: vec![],
        }
    }

    fn record(&mut self, index: &usize) {
        self.index_history.push(*index);
    }

    fn length(&self) -> usize {
        self.index_history.len()
    }

    fn contains(&self, index: &usize) -> bool {
        self.index_history.contains(index)
    }
}

struct BootDebugger;

impl BootDebugger {
    fn new() -> Self {
        BootDebugger
    }

    fn get_accumulator_value_before_repeated_instruction(
        &self,
        boot_instructions: Vec<BootInstruction>,
    ) -> anyhow::Result<i64> {
        let mut execution_record = ExecutionHistory::new();
        let expected_max_execution_length = boot_instructions.len();
        let mut executor = Executor::new(boot_instructions);

        while execution_record.length() <= expected_max_execution_length {
            let next_instruction_index = executor.next_instruction_index();
            if !execution_record.contains(&next_instruction_index) {
                executor.apply_next_instruction()?;
                execution_record.record(&next_instruction_index);
            } else {
                break;
            }
        }

        Ok(executor.accumulated_value())
    }
}

pub fn get_accumulator_value_before_repeated_instruction(
    boot_instruction_strings: Vec<String>,
) -> anyhow::Result<i64> {
    let boot_instructions = boot_instruction_strings
        .iter()
        .map(|instruction_string| instruction_string.parse())
        .collect::<anyhow::Result<Vec<BootInstruction>>>()?;
    Ok(BootDebugger::new().get_accumulator_value_before_repeated_instruction(boot_instructions)?)
}

#[cfg(test)]
mod tests {
    use spectral::prelude::*;

    use super::*;

    #[test]
    fn gets_accumulator_value_before_repeated_instruction() {
        let boot_instruction_strings = vec![
            "nop +0", "acc +1", "jmp +4", "acc +3", "jmp -3", "acc -99", "acc +1", "jmp -4",
            "acc +6",
        ]
        .iter()
        .map(ToString::to_string)
        .collect();

        assert_that(
            &get_accumulator_value_before_repeated_instruction(boot_instruction_strings).unwrap(),
        )
        .is_equal_to(5);
    }
}
