use std::{cmp, mem};
use std::borrow::Cow;
use std::io::Write;
use std::process::exit;
use log::error;
use pest::error::{Error, LineColLocation};
use pest::error::ErrorVariant::ParsingError;
use termcolor::{Buffer, BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};
use crate::parser::types::Rule;

pub struct ErrorGen {
    whammy_path: String,
    script_text: String,
    max_errors: i32,
    errors: Vec<WhammError>,
    num_errors: i32,
    pub too_many: bool,
    no_errors: bool
}
impl ErrorGen {
    pub fn new(whammy_path: String, script_text: String, max_errors: i32) -> Self {
        Self {
            whammy_path,
            script_text,
            max_errors,
            errors: vec![],
            num_errors: 0,
            too_many: false,
            no_errors: true
        }
    }

    pub fn set_script_text(&mut self, script_text: String) {
        self.script_text = script_text;
    }

    pub fn report(&mut self) {
        // Report the most-recent error first
        self.errors.iter().for_each(|mut error| {
            error.report(&self.script_text, &self.whammy_path);
        });
        self.errors.clear();
    }

    pub fn fatal_report(&mut self, context: &str) {
        if self.no_errors {
            return;
        }
        self.report();
        error!("{context}: Expected no errors.");
        exit(1);
    }

    // ======================
    // == Error Generators ==
    // ======================

    pub fn unexpected_error(&mut self, fatal: bool, message: Option<String>) {
        let err = WhammError {
            fatal,
            ty: ErrorType::Error {
                message
            },
            line_col: None,
            line: None,
            line2: None
        };
        self.add_error(err);

        if fatal {
            self.fatal_report("Fatal");
        }
    }

    pub fn parse_error(&mut self, fatal: bool, message: Option<String>, line_col: LineColLocation,
                       positives: Vec<Rule>, negatives: Vec<Rule>) {
        let err = WhammError {
            fatal,
            ty: ErrorType::ParsingError {
                positives,
                negatives,
                message
            },
            line_col: Some(line_col),
            line: None,
            line2: None
        };
        self.add_error(err);

        if fatal {
            self.fatal_report("Fatal");
        }
    }

    pub fn type_check_error(&mut self, fatal: bool, message: String, line_col: LineColLocation) {
        let err = WhammError {
            fatal,
            ty: ErrorType::TypeCheckError {
                message
            },
            line_col: Some(line_col),
            line: None,
            line2: None
        };
        self.add_error(err);

        if fatal {
            self.fatal_report("Fatal");
        }
    }

    pub fn pest_err(&mut self, e: Error<Rule>) {
        let line = e.line().to_string();

        // calculate `line2`
        let line2 = if let LineColLocation::Span(..) = &e.line_col {
            // pull out the `line2` from the error msg
            let orig_msg = e.to_string();
            // get last line that starts with a number
            // See code the following code for why we can do this:
            // https://github.com/pest-parser/pest/blob/master/pest/src/error.rs#L612
            let mut lines = orig_msg.lines();
            if let Some(line) = lines.rfind(|line| {
                line.as_bytes()[0].is_ascii_digit()
            }) {
                Some(line.to_string())
            } else {
                None
            }
        } else {
            None
        };

        let error = if let ParsingError {positives, negatives} = &e.variant {
            WhammError {
                fatal: false,
                ty: ErrorType::ParsingError {
                    positives: positives.clone(),
                    negatives: negatives.clone(),
                    message: None
                },
                line_col: Some(e.line_col.clone()),
                line: Some(line),
                line2
            }
        } else {
            WhammError {
                fatal: false,
                ty: ErrorType::Error {
                    message: None
                },
                line_col: Some(e.line_col.clone()),
                line: Some(line),
                line2
            }
        };
        self.add_error(error);
    }

    fn add_error(&mut self, error: WhammError) {
        self.errors.push(error);
        self.inc_errors();
    }

    fn inc_errors(&mut self) {
        self.num_errors += 1;
        self.no_errors = false;
        if self.num_errors >= self.max_errors {
            self.too_many = true;
        }
    }
}

pub struct WhammError {
    pub fatal: bool,
    /// Line/column within the input string
    pub line_col: Option<LineColLocation>,
    // range: FileRange,
    pub ty: ErrorType,
    // /// Location within the input string
    // pub location: InputLocation,
    pub line: Option<String>,
    pub line2: Option<String>,
    // error: String,
}
impl WhammError {

    // report this error to the console, including color highlighting
    pub fn report(&mut self, script: &String, whammy_path: &String) {
        if self.line.is_none() {
            self.define_lines(script);
        }

        let writer = BufferWriter::stderr(ColorChoice::Always);
        let mut buffer = writer.buffer();
        if let Some(line_col) = self.line_col.clone() {
            if let Some(line) = &self.line {

                // define common vars for printing
                let ls = self.start().0;
                let underline = self.underline();
                let message = self.ty.message();
                if let (LineColLocation::Span(_, end), Some(ref line2)) = (line_col, &self.line2) {
                    let has_line_gap = end.0 - self.start().0 > 1;

                    // define common vars for printing
                    let le = end.0;
                    if has_line_gap {
                        self.print_preamble(whammy_path, 4, &mut buffer);
                        self.print_empty(4, &mut buffer);
                        self.print_numbered_line(ls, line, &mut buffer);
                        self.print_line("...", false, 4, &mut buffer);
                        self.print_numbered_line(le, line2, &mut buffer);
                        self.print_line(&format!("{underline} {message}"), true, 4, &mut buffer);
                        self.print_empty(4, &mut buffer);
                    } else {
                        self.print_preamble(whammy_path, 4, &mut buffer);
                        self.print_empty(4, &mut buffer);
                        self.print_numbered_line(ls, line, &mut buffer);
                        self.print_numbered_line(le, line2, &mut buffer);
                        self.print_line(&format!("{underline} {message}"), true, 4, &mut buffer);
                        self.print_empty(4, &mut buffer);
                    }
                } else {
                    self.print_preamble(whammy_path, 0, &mut buffer);
                    self.print_empty(0, &mut buffer);
                    self.print_numbered_line(ls, line, &mut buffer);
                    self.print_line(&format!("{underline} {message}"), true, 0, &mut buffer);
                    self.print_empty(0, &mut buffer);
                };
            }
        } else {
            // This error isn't tied to a specific code location
            set_bold(true, &mut buffer);
            red(format!("error[{}]", self.ty.name()), &mut buffer);
            white(format!(": {}\n", self.ty.message()), &mut buffer);
            set_bold(false, &mut buffer);
            blue(format!(" --> "), &mut buffer);
            blue(format!("{whammy_path}\n\n"), &mut buffer);
        }
        writer.print(&buffer).expect("Uh oh, something went wrong while printing to terminal");
        buffer.reset().expect("Uh oh, something went wrong while printing to terminal");
    }
    fn define_lines(&mut self, script: &String) {
        match self.line_col {
            Some(LineColLocation::Pos((line_no, ..))) => {
                if let Some(script_line) = script.lines().nth(line_no) {
                    self.line = Some(script_line.to_string());
                }
            }
            Some(LineColLocation::Span((s0_line, ..), (s1_line, ..))) => {
                if let Some(script_line) = script.lines().nth(s0_line) {
                    self.line = Some(script_line.to_string());
                }
                if let Some(script_line) = script.lines().nth(s1_line) {
                    self.line = Some(script_line.to_string());
                }
            }
        }
    }

    fn print_preamble(&self, whammy_path: &String, extra_spaces: usize, buffer: &mut Buffer) {
        let s = self.spacing();
        let ls = self.start().0;
        let c = self.start().1;
        let spaces = " ".repeat(extra_spaces);

        blue(format!("{s}{spaces}--> "), buffer);
        blue(format!("{whammy_path}:"), buffer);
        blue(format!("{ls}:{c}\n"), buffer);
    }

    fn print_numbered_line(&self, l: usize, line: &String, buffer: &mut Buffer) {
        let s = self.spacing();
        let w = s.len();
        blue(format!("{l:w$} | "), buffer);
        white(format!("{line}\n"), buffer);
    }

    fn print_line(&self, line: &str, is_err: bool, extra_spaces: usize, buffer: &mut Buffer) {
        let s = self.spacing();
        let spaces = " ".repeat(extra_spaces);
        blue(format!("{s}{spaces} | "), buffer);
        if is_err {
            red(format!("{line}\n"), buffer);
        } else {
            white(format!("{line}\n"), buffer);
        }
    }

    fn print_empty(&self, extra_spaces: usize, buffer: &mut Buffer) {
        self.print_line("", false, extra_spaces, buffer);
    }

    fn spacing(&self) -> String {
        let line = match self.line_col {
            Some(LineColLocation::Pos((line, _))) => line,
            Some(LineColLocation::Span((start_line, _), (end_line, _))) => {
                cmp::max(start_line, end_line)
            }
            _ => {
                return "".to_string();
            }
        };

        let line_str_len = format!("{}", line).len();

        let mut spacing = String::new();
        for _ in 0..line_str_len {
            spacing.push(' ');
        }

        spacing
    }

    fn underline(&self) -> String {
        let mut underline = String::new();

        let mut start = self.start().1;
        let end = match self.line_col {
            Some(LineColLocation::Span(_, (_, mut end))) => {
                let inverted_cols = start > end;
                if inverted_cols {
                    mem::swap(&mut start, &mut end);
                    start -= 1;
                    end += 1;
                }

                Some(end)
            }
            _ => None,
        };
        let offset = start - 1;

        if let Some(line) = &self.line {
            let line_chars = line.chars();

            for c in line_chars.take(offset) {
                match c {
                    '\t' => underline.push('\t'),
                    _ => underline.push(' '),
                }
            }
        }

        if let Some(end) = end {
            underline.push('^');
            if end - start > 1 {
                for _ in 2..(end - start) {
                    underline.push('-');
                }
                underline.push('^');
            }
        } else {
            underline.push_str("^---")
        }

        underline
    }

    fn start(&self) -> (usize, usize) {
        match self.line_col {
            Some(LineColLocation::Pos(line_col)) => line_col,
            Some(LineColLocation::Span(start_line_col, _)) => start_line_col,
            _ => {
                unreachable!()
            }
        }
    }
}

pub enum ErrorType {
    /// Generated parsing error with expected and unexpected `Rule`s
    ParsingError {
        /// Positive attempts
        positives: Vec<Rule>,
        /// Negative attempts
        negatives: Vec<Rule>,
        message: Option<String>
    },
    /// Error during type checking
    TypeCheckError {
        message: String
    },
    Error {
        message: Option<String>
    }
}
impl ErrorType {
    pub fn name(&self) -> &str {
        match self {
            ErrorType::ParsingError {..} => "ParsingError",
            ErrorType::TypeCheckError {..} => "TypeCheckError",
            ErrorType::Error {..} => "GeneralError"
        }
    }
    pub fn message(&self) -> Cow<'_, str> {
        match self {
            ErrorType::ParsingError {
                ref positives,
                ref negatives,
                ref message
            } => Cow::Owned(Self::parsing_error_message(message, positives, negatives, |r| {
                format!("{:?}", r)
            })),
            ErrorType::TypeCheckError { ref message } => Cow::Borrowed(message),
            ErrorType::Error { ref message } => {
                if let Some(msg) = message {
                    Cow::Borrowed(msg)
                } else {
                    Cow::Borrowed("An error occurred.")
                }
            },
        }
    }

    fn parsing_error_message<F>(message: &Option<String>, positives: &[Rule], negatives: &[Rule], mut f: F) -> String
        where F: FnMut(&Rule) -> String,
    {
        let preamble = if let Some(msg) = message {
            format!("{msg} -- ")
        } else {
            "".to_string()
        };
        match (negatives.is_empty(), positives.is_empty()) {
            (false, false) => format!(
                "{}unexpected {}; expected {}",
                preamble,
                ErrorType::enumerate(negatives, &mut f),
                ErrorType::enumerate(positives, &mut f)
            ),
            (false, true) => format!("{}unexpected {}", preamble, ErrorType::enumerate(negatives, &mut f)),
            (true, false) => format!("{}expected {}", preamble, ErrorType::enumerate(positives, &mut f)),
            (true, true) => {
                if preamble.is_empty() {
                    "unknown parsing error".to_owned()
                } else {
                    preamble
                }
            },
        }
    }

    fn enumerate<F>(rules: &[Rule], f: &mut F) -> String
        where
            F: FnMut(&Rule) -> String,
    {
        match rules.len() {
            1 => f(&rules[0]),
            2 => format!("{} or {}", f(&rules[0]), f(&rules[1])),
            l => {
                let non_separated = f(&rules[l - 1]);
                let separated = rules
                    .iter()
                    .take(l - 1)
                    .map(f)
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{}, or {}", separated, non_separated)
            }
        }
    }
}

// ===========================
// = Terminal Printing Logic =
// ===========================

fn set_bold(yes: bool, buffer: &mut Buffer) {
    let write_err = "Uh oh, something went wrong while printing to terminal";
    buffer.set_color(ColorSpec::new().set_bold(yes)).expect(write_err);
}

fn color(s: String, buffer: &mut Buffer, c: Color) {
    let write_err = "Uh oh, something went wrong while printing to terminal";
    buffer.set_color(ColorSpec::new().set_fg(Some(c))).expect(write_err);
    write!(buffer, "{}", s.as_str()).expect(write_err);
}

fn black(s: String, buffer: &mut Buffer) {
    color(s, buffer, Color::Black)
}
fn blue(s: String, buffer: &mut Buffer) {
    color(s, buffer, Color::Blue)
}
fn cyan(s: String, buffer: &mut Buffer) {
    color(s, buffer, Color::Cyan)
}
fn green(s: String, buffer: &mut Buffer) {
    color(s, buffer, Color::Green)
}
fn magenta(s: String, buffer: &mut Buffer) {
    color(s, buffer, Color::Magenta)
}
fn red(s: String, buffer: &mut Buffer) {
    color(s, buffer, Color::Red)
}
fn white(s: String, buffer: &mut Buffer) {
    color(s, buffer, Color::Rgb(193,193,193))
}
fn yellow(s: String, buffer: &mut Buffer) {
    color(s, buffer, Color::Yellow)
}