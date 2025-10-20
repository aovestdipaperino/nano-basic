use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::interpreter::Interpreter;

const C64_BLUE: Color = Color::Rgb(64, 50, 133);
const C64_LIGHT_BLUE: Color = Color::Rgb(122, 191, 199);
const WIDTH: usize = 40;
const HEIGHT: usize = 25;

pub struct App {
    pub input: String,
    pub output: Vec<String>,
    pub interpreter: Interpreter,
    pub cursor_pos: usize,
}

impl App {
    pub fn new() -> Self {
        let mut output = Vec::new();
        output.push("    **** COMMODORE 64 BASIC V2 ****".to_string());
        output.push("".to_string());
        output.push(" 64K RAM SYSTEM  38911 BASIC BYTES FREE".to_string());
        output.push("".to_string());
        output.push("READY.".to_string());

        App {
            input: String::new(),
            output,
            interpreter: Interpreter::new(),
            cursor_pos: 0,
        }
    }

    pub fn handle_char(&mut self, c: char) {
        // C64 only supports uppercase
        let c = c.to_uppercase().next().unwrap_or(c);
        self.input.push(c);
        self.cursor_pos += 1;
    }

    pub fn handle_backspace(&mut self) {
        if !self.input.is_empty() {
            self.input.pop();
            if self.cursor_pos > 0 {
                self.cursor_pos -= 1;
            }
        }
    }

    pub fn handle_enter(&mut self) -> bool {
        let input = self.input.clone();
        self.output.push(format!("{}", input));
        self.input.clear();
        self.cursor_pos = 0;

        // Process the command
        let result = self.interpreter.execute(&input);
        match result {
            Ok(output) => {
                if !output.is_empty() {
                    for line in output {
                        self.output.push(line);
                    }
                }
                self.output.push("READY.".to_string());
            }
            Err(err) => {
                // Check for QUIT command
                if err == "QUIT" {
                    return true; // Signal to quit
                }
                self.output.push(format!("?{} ERROR", err.to_uppercase()));
                self.output.push("READY.".to_string());
            }
        }

        // Keep only last HEIGHT-2 lines (leave room for input and border)
        if self.output.len() > HEIGHT - 2 {
            let skip = self.output.len() - (HEIGHT - 2);
            self.output = self.output[skip..].to_vec();
        }

        false // Don't quit
    }
}

pub fn draw(f: &mut Frame, app: &App) {
    // Create a 40x25 character area
    let size = f.area();

    // Calculate centered area for 40x25 display
    let char_width = 2; // Approximate character width ratio
    let display_width = WIDTH as u16 * char_width;
    let display_height = HEIGHT as u16;

    let horizontal_margin = size.width.saturating_sub(display_width) / 2;
    let vertical_margin = size.height.saturating_sub(display_height) / 2;

    let display_area = Rect {
        x: horizontal_margin,
        y: vertical_margin,
        width: display_width.min(size.width),
        height: display_height.min(size.height),
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(display_area);

    // Output area
    let output_text: Vec<Line> = app
        .output
        .iter()
        .map(|line| {
            Line::from(Span::styled(
                format!("{:width$}", line, width = WIDTH),
                Style::default().fg(C64_LIGHT_BLUE),
            ))
        })
        .collect();

    let output = Paragraph::new(output_text)
        .style(Style::default().bg(C64_BLUE).fg(C64_LIGHT_BLUE))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(C64_LIGHT_BLUE))
                .title(" COMMODORE 64 BASIC ")
                .title_style(Style::default().fg(C64_LIGHT_BLUE).add_modifier(Modifier::BOLD)),
        )
        .wrap(Wrap { trim: false });

    f.render_widget(output, chunks[0]);

    // Input area
    let input_display = format!("{}_", app.input);
    let input = Paragraph::new(Line::from(Span::styled(
        format!("{:width$}", input_display, width = WIDTH),
        Style::default().fg(C64_LIGHT_BLUE),
    )))
    .style(Style::default().bg(C64_BLUE).fg(C64_LIGHT_BLUE))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(C64_LIGHT_BLUE)),
    );

    f.render_widget(input, chunks[1]);
}
