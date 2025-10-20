//! BASIC interpreter core functionality.
//!
//! This module implements the interpreter for Commodore 64 BASIC V2, including
//! program storage, variable management, control flow (GOTO, GOSUB, FOR/NEXT),
//! and execution of BASIC statements. It also handles loading and saving programs
//! in the C64 .PRG format.

use crate::expression::ExpressionParser;
use crate::tokens::TokenTable;
use std::collections::{BTreeMap, HashMap};
use std::fs;

#[derive(Debug)]
pub struct Interpreter {
    token_table: TokenTable,
    /// Program lines stored by line number
    program: BTreeMap<u16, Vec<u8>>,
    /// Variables (numeric and string)
    pub variables: HashMap<String, Value>,
    /// GOSUB return stack (line numbers)
    return_stack: Vec<u16>,
    /// Expression parser
    expr_parser: ExpressionParser,
    /// FOR loop stack
    for_stack: Vec<ForLoop>,
}

#[derive(Clone, Debug)]
pub enum Value {
    Number(f64),
    String(String),
}

#[derive(Clone, Debug)]
struct ForLoop {
    var_name: String,
    end_value: f64,
    step: f64,
    return_line: u16,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            token_table: TokenTable::new(),
            program: BTreeMap::new(),
            variables: HashMap::new(),
            return_stack: Vec::new(),
            expr_parser: ExpressionParser::new(),
            for_stack: Vec::new(),
        }
    }

    /// Executes a BASIC command or adds a program line.
    ///
    /// If the input starts with a line number, it adds or updates that program line.
    /// If the input is a line number alone, it deletes that line. Otherwise, it executes
    /// the input as an immediate command.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let mut interp = Interpreter::new();
    /// // Add a program line
    /// interp.execute("10 PRINT \"HELLO\"")?;
    /// // Execute immediate command
    /// let output = interp.execute("PRINT 2+2")?;
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error string if the command fails (e.g., syntax error, file not found).
    pub fn execute(&mut self, input: &str) -> Result<Vec<String>, String> {
        let trimmed = input.trim();
        if trimmed.is_empty() {
            return Ok(Vec::new());
        }

        // Check if line starts with a number (program line)
        let parts: Vec<&str> = trimmed.splitn(2, ' ').collect();
        if let Ok(line_num) = parts[0].parse::<u16>() {
            // It's a program line
            if parts.len() == 1 {
                // Just a line number - delete that line
                self.program.remove(&line_num);
                return Ok(Vec::new());
            } else {
                // Add/replace line
                let line_content = parts[1];
                let tokenized = self.token_table.tokenize(line_content);
                self.program.insert(line_num, tokenized);
                return Ok(Vec::new());
            }
        }

        // It's an immediate command
        self.execute_command(trimmed)
    }

    fn execute_command(&mut self, cmd: &str) -> Result<Vec<String>, String> {
        let upper = cmd.to_uppercase();
        let parts: Vec<&str> = upper.split_whitespace().collect();

        if parts.is_empty() {
            return Ok(Vec::new());
        }

        match parts[0] {
            "LIST" => self.cmd_list(),
            "RUN" => self.cmd_run(),
            "NEW" => self.cmd_new(),
            "CLR" => self.cmd_clr(),
            "QUIT" | "EXIT" | "BYE" => Err("QUIT".to_string()),
            "LOAD" => {
                if parts.len() < 2 {
                    return Err("SYNTAX".to_string());
                }
                let filename = parts[1].trim_matches('"');
                self.cmd_load(filename)
            }
            "SAVE" => {
                if parts.len() < 2 {
                    return Err("SYNTAX".to_string());
                }
                let filename = parts[1].trim_matches('"');
                self.cmd_save(filename)
            }
            "PRINT" | "?" => {
                let rest = if upper.starts_with("PRINT ") {
                    &cmd[6..]
                } else if upper.starts_with("? ") {
                    &cmd[2..]
                } else {
                    ""
                };
                self.cmd_print(rest)
            }
            _ => {
                // Try to execute as a statement (like LET X=5)
                let result = self.execute_statement(cmd, None)?;
                Ok(result.0)
            }
        }
    }

    fn cmd_list(&self) -> Result<Vec<String>, String> {
        let mut output = Vec::new();
        for (line_num, tokenized) in &self.program {
            let detokenized = self.token_table.detokenize(tokenized);
            output.push(format!("{} {}", line_num, detokenized));
        }
        Ok(output)
    }

    fn cmd_run(&mut self) -> Result<Vec<String>, String> {
        let mut output = Vec::new();
        self.variables.clear();
        self.return_stack.clear();
        self.for_stack.clear();

        let line_numbers: Vec<u16> = self.program.keys().copied().collect();
        let mut pc = 0; // Program counter index

        while pc < line_numbers.len() {
            let line_num = line_numbers[pc];
            if let Some(tokenized) = self.program.get(&line_num) {
                let detokenized = self.token_table.detokenize(tokenized);
                let result = self.execute_statement(&detokenized, Some(line_num))?;

                // Check for GOTO or GOSUB
                if let Some(jump_line) = result.1 {
                    // Find the line number in our list
                    if let Some(new_pc) = line_numbers.iter().position(|&x| x == jump_line) {
                        pc = new_pc;
                        continue;
                    } else {
                        return Err("UNDEF'D STATEMENT".to_string());
                    }
                }

                output.extend(result.0);
            }
            pc += 1;
        }

        Ok(output)
    }

    fn cmd_new(&mut self) -> Result<Vec<String>, String> {
        self.program.clear();
        self.variables.clear();
        self.return_stack.clear();
        self.for_stack.clear();
        Ok(Vec::new())
    }

    fn cmd_clr(&mut self) -> Result<Vec<String>, String> {
        self.variables.clear();
        self.return_stack.clear();
        self.for_stack.clear();
        Ok(Vec::new())
    }

    fn cmd_load(&mut self, filename: &str) -> Result<Vec<String>, String> {
        let filepath = if filename.ends_with(".prg") {
            filename.to_string()
        } else {
            format!("{}.prg", filename)
        };

        let data = fs::read(&filepath).map_err(|_| "FILE NOT FOUND".to_string())?;
        self.load_prg(&data)?;
        Ok(vec![format!("LOADING {}", filename.to_uppercase())])
    }

    fn cmd_save(&self, filename: &str) -> Result<Vec<String>, String> {
        let filepath = if filename.ends_with(".prg") {
            filename.to_string()
        } else {
            format!("{}.prg", filename)
        };

        let data = self.save_prg()?;
        fs::write(&filepath, data).map_err(|_| "FILE ERROR".to_string())?;
        Ok(vec![format!("SAVING {}", filename.to_uppercase())])
    }

    fn cmd_print(&mut self, expr: &str) -> Result<Vec<String>, String> {
        if expr.is_empty() {
            return Ok(vec!["".to_string()]);
        }

        // Parse and evaluate expression
        let value = self.expr_parser.parse_and_eval(expr, &self.variables)?;
        let output = match value {
            Value::Number(n) => {
                if n.fract() == 0.0 && n.abs() < 1e10 {
                    format!(" {}", n as i64)
                } else {
                    format!(" {}", n)
                }
            }
            Value::String(s) => s,
        };

        Ok(vec![output])
    }

    /// Execute a statement and return (output, optional_jump_line)
    fn execute_statement(
        &mut self,
        statement: &str,
        current_line: Option<u16>,
    ) -> Result<(Vec<String>, Option<u16>), String> {
        let trimmed = statement.trim();
        if trimmed.is_empty() {
            return Ok((Vec::new(), None));
        }

        let upper = trimmed.to_uppercase();

        // Handle LET (explicit or implicit)
        if upper.starts_with("LET ") || trimmed.contains('=') {
            let assign_part = if upper.starts_with("LET ") {
                &trimmed[4..]
            } else {
                trimmed
            };

            if let Some(eq_pos) = assign_part.find('=') {
                let var_name = assign_part[..eq_pos].trim().to_uppercase();
                let expr = &assign_part[eq_pos + 1..];

                let value = self.expr_parser.parse_and_eval(expr, &self.variables)?;
                self.variables.insert(var_name, value);
                return Ok((Vec::new(), None));
            }
        }

        // Handle PRINT
        if upper.starts_with("PRINT ") || upper.starts_with("? ") {
            let rest = if upper.starts_with("PRINT ") {
                &trimmed[6..]
            } else {
                &trimmed[2..]
            };
            let output = self.cmd_print(rest)?;
            return Ok((output, None));
        }

        // Handle GOTO
        if let Some(stripped) = upper.strip_prefix("GOTO ") {
            let line_str = stripped.trim();
            let target = line_str
                .parse::<u16>()
                .map_err(|_| "SYNTAX ERROR".to_string())?;
            return Ok((Vec::new(), Some(target)));
        }

        // Handle GOSUB
        if let Some(stripped) = upper.strip_prefix("GOSUB ") {
            let line_str = stripped.trim();
            let target = line_str
                .parse::<u16>()
                .map_err(|_| "SYNTAX ERROR".to_string())?;
            if let Some(current) = current_line {
                self.return_stack.push(current);
            }
            return Ok((Vec::new(), Some(target)));
        }

        // Handle RETURN
        if upper == "RETURN" {
            if let Some(return_line) = self.return_stack.pop() {
                // Return to line after GOSUB
                let line_numbers: Vec<u16> = self.program.keys().copied().collect();
                if let Some(pos) = line_numbers.iter().position(|&x| x == return_line) {
                    if pos + 1 < line_numbers.len() {
                        return Ok((Vec::new(), Some(line_numbers[pos + 1])));
                    }
                }
            }
            return Ok((Vec::new(), None));
        }

        // Handle IF/THEN
        if upper.starts_with("IF ") {
            if let Some(then_pos) = upper.find(" THEN ") {
                let condition = &trimmed[3..then_pos];
                let then_part = &trimmed[then_pos + 6..];

                let value = self
                    .expr_parser
                    .parse_and_eval(condition, &self.variables)?;
                let is_true = match value {
                    Value::Number(n) => n != 0.0,
                    Value::String(_) => return Err("TYPE MISMATCH".to_string()),
                };

                if is_true {
                    // Check if THEN part is a line number or statement
                    if let Ok(line_num) = then_part.trim().parse::<u16>() {
                        return Ok((Vec::new(), Some(line_num)));
                    } else {
                        return self.execute_statement(then_part, current_line);
                    }
                }
            }
            return Ok((Vec::new(), None));
        }

        // Handle FOR
        if upper.starts_with("FOR ") {
            // FOR I=1 TO 10 STEP 2
            let rest = &trimmed[4..];
            let parts: Vec<&str> = rest.split_whitespace().collect();

            if parts.len() < 4 {
                return Err("SYNTAX ERROR".to_string());
            }

            // Parse I=1
            let var_assign = parts[0];
            if let Some(eq_pos) = var_assign.find('=') {
                let var_name = var_assign[..eq_pos].trim().to_uppercase();
                let start_expr = &var_assign[eq_pos + 1..];
                let start_val = self
                    .expr_parser
                    .parse_and_eval(start_expr, &self.variables)?;

                if let Value::Number(start) = start_val {
                    // Find TO
                    if let Some(to_pos) = rest.to_uppercase().find(" TO ") {
                        let end_expr = &rest[to_pos + 4..];

                        // Check for STEP
                        let (end_expr, step) =
                            if let Some(step_pos) = end_expr.to_uppercase().find(" STEP ") {
                                let step_expr = &end_expr[step_pos + 6..];
                                let step_val = self
                                    .expr_parser
                                    .parse_and_eval(step_expr, &self.variables)?;
                                if let Value::Number(s) = step_val {
                                    (&end_expr[..step_pos], s)
                                } else {
                                    return Err("TYPE MISMATCH".to_string());
                                }
                            } else {
                                (end_expr, 1.0)
                            };

                        let end_val = self.expr_parser.parse_and_eval(end_expr, &self.variables)?;
                        if let Value::Number(end) = end_val {
                            // Set loop variable
                            self.variables
                                .insert(var_name.clone(), Value::Number(start));

                            // Push loop info
                            if let Some(line) = current_line {
                                self.for_stack.push(ForLoop {
                                    var_name,
                                    end_value: end,
                                    step,
                                    return_line: line,
                                });
                            }
                        }
                    }
                }
            }
            return Ok((Vec::new(), None));
        }

        // Handle NEXT
        if upper.starts_with("NEXT") {
            if let Some(loop_info) = self.for_stack.last().cloned() {
                // Increment loop variable
                if let Some(Value::Number(current)) = self.variables.get(&loop_info.var_name) {
                    let new_val = current + loop_info.step;

                    // Check if loop should continue
                    let should_continue = if loop_info.step > 0.0 {
                        new_val <= loop_info.end_value
                    } else {
                        new_val >= loop_info.end_value
                    };

                    if should_continue {
                        self.variables
                            .insert(loop_info.var_name.clone(), Value::Number(new_val));
                        // Jump to line AFTER the FOR statement
                        let line_numbers: Vec<u16> = self.program.keys().copied().collect();
                        if let Some(pos) = line_numbers
                            .iter()
                            .position(|&x| x == loop_info.return_line)
                        {
                            if pos + 1 < line_numbers.len() {
                                return Ok((Vec::new(), Some(line_numbers[pos + 1])));
                            }
                        }
                        return Ok((Vec::new(), Some(loop_info.return_line)));
                    } else {
                        self.for_stack.pop();
                    }
                }
            }
            return Ok((Vec::new(), None));
        }

        // Handle REM
        if upper.starts_with("REM ") || upper == "REM" {
            return Ok((Vec::new(), None));
        }

        // Handle END
        if upper == "END" {
            return Ok((Vec::new(), None));
        }

        // Handle STOP
        if upper == "STOP" {
            return Err("BREAK".to_string());
        }

        Ok((Vec::new(), None))
    }

    fn load_prg(&mut self, data: &[u8]) -> Result<(), String> {
        if data.len() < 2 {
            return Err("INVALID FILE".to_string());
        }

        // Skip the first 2 bytes (0x0801), which is the C64 BASIC load address
        let mut pos = 2;
        self.program.clear();

        while pos < data.len() {
            if pos + 1 < data.len() && data[pos] == 0 && data[pos + 1] == 0 {
                break;
            }

            if pos + 3 >= data.len() {
                break;
            }
            pos += 2;

            let line_num = data[pos] as u16 | ((data[pos + 1] as u16) << 8);
            pos += 2;

            let mut tokens = Vec::new();
            while pos < data.len() && data[pos] != 0 {
                tokens.push(data[pos]);
                pos += 1;
            }

            tokens.push(0);
            self.program.insert(line_num, tokens);
            pos += 1;
        }

        Ok(())
    }

    fn save_prg(&self) -> Result<Vec<u8>, String> {
        let mut data = Vec::new();
        // Write the C64 BASIC load address (0x0801)
        data.push(0x01);
        data.push(0x08);

        // Starting address for BASIC programs on the C64
        let mut addr = 0x0801u16;

        for (line_num, tokens) in &self.program {
            let line_len = 2 + 2 + tokens.len();
            let next_addr = addr + line_len as u16;

            data.push((next_addr & 0xFF) as u8);
            data.push((next_addr >> 8) as u8);
            data.push((*line_num & 0xFF) as u8);
            data.push((*line_num >> 8) as u8);
            data.extend_from_slice(tokens);

            addr = next_addr;
        }

        data.push(0x00);
        data.push(0x00);

        Ok(data)
    }
}
