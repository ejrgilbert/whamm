use crate::common::terminal::{blue, red, white};
use crate::parser::types::{Location, Rule};
use log::error;
use pest::error::ErrorVariant::ParsingError;
use pest::error::{Error, LineColLocation};
use std::borrow::Cow;
use std::process::exit;
use std::{cmp, mem};
use termcolor::{Buffer, BufferWriter, ColorChoice, WriteColor};

const ERR_UNDERLINE_CHAR: char = '^';
const INFO_UNDERLINE_CHAR: char = '-';

pub struct ErrorGen {
    script_path: String,
    script_text: String,
    max_errors: i32,
    errors: Vec<WhammError>,
    num_errors: i32,
    pub too_many: bool,
    pub has_errors: bool,
}
impl ErrorGen {
    pub fn new(script_path: String, script_text: String, max_errors: i32) -> Self {
        Self {
            script_path,
            script_text,
            max_errors,
            errors: vec![],
            num_errors: 0,
            too_many: false,
            has_errors: false,
        }
    }

    pub fn add_error(&mut self, error: WhammError) {
        let fatal = error.fatal;
        self.errors.push(error);
        self.inc_errors();

        if fatal {
            self.fatal_report("Fatal");
        }
    }

    pub fn add_errors(&mut self, errors: Vec<WhammError>) {
        for error in errors {
            self.add_error(error);
        }

        self.check_too_many();
    }

    pub fn set_script_text(&mut self, script_text: String) {
        self.script_text = script_text;
    }

    pub fn report(&mut self) {
        // Report the most-recent error first
        self.errors.iter_mut().for_each(|error| {
            error.report(&self.script_text, &self.script_path);
        });
        self.errors.clear();
    }

    pub fn fatal_report(&mut self, context: &str) {
        if !&self.has_errors {
            return;
        }
        self.report();
        error!("{context}: Expected no errors.");
        exit(1);
    }

    pub fn check_has_errors(&mut self) {
        if self.has_errors {
            self.report();
            exit(1);
        }
    }

    pub fn check_too_many(&mut self) {
        if self.too_many {
            self.report();
            exit(1);
        }
    }

    // ======================
    // == Error Generators ==
    // ======================

    pub fn get_parse_error(
        fatal: bool,
        message: Option<String>,
        line_col: Option<LineColLocation>,
        positives: Vec<Rule>,
        negatives: Vec<Rule>,
    ) -> WhammError {
        if let Some(line_col) = line_col {
            WhammError {
                fatal,
                ty: ErrorType::ParsingError {
                    positives,
                    negatives,
                    message: message.clone(),
                },
                err_loc: Some(CodeLocation {
                    is_err: true,
                    message,
                    line_col,
                    line_str: None,
                    line2_str: None,
                }),
                info_loc: None,
            }
        } else {
            WhammError {
                fatal,
                ty: ErrorType::ParsingError {
                    positives,
                    negatives,
                    message: message.clone(),
                },
                err_loc: None,
                info_loc: None,
            }
        }
    }

    pub fn parse_error(
        &mut self,
        fatal: bool,
        message: Option<String>,
        line_col: Option<LineColLocation>,
        positives: Vec<Rule>,
        negatives: Vec<Rule>,
    ) {
        let err = Self::get_parse_error(fatal, message, line_col, positives, negatives);
        self.add_error(err);
    }

    pub fn get_duplicate_identifier_error(
        fatal: bool,
        duplicated_id: String,
        err_line_col: Option<LineColLocation>,
        info_line_col: Option<LineColLocation>,
    ) -> WhammError {
        let err_loc = err_line_col.map(|err_line_col| CodeLocation {
            is_err: true,
            message: Some(format!("duplicate definitions for `{}`", duplicated_id)),
            line_col: err_line_col,
            line_str: None,
            line2_str: None,
        });
        let info_loc = info_line_col.map(|info_line_col| CodeLocation {
            is_err: false,
            message: Some(format!("other definition for `{}`", duplicated_id)),
            line_col: info_line_col,
            line_str: None,
            line2_str: None,
        });

        WhammError {
            fatal,
            ty: ErrorType::DuplicateIdentifierError {
                duplicated_id: duplicated_id.clone(),
            },
            err_loc,
            info_loc,
        }
    }

    pub fn get_duplicate_identifier_error_from_loc(
        fatal: bool,
        duplicated_id: String,
        err_loc: &Option<Location>,
        info_loc: &Option<Location>,
    ) -> WhammError {
        let err_loc = err_loc.as_ref().map(|err_loc| err_loc.line_col.clone());
        let info_loc = info_loc.as_ref().map(|info_loc| info_loc.line_col.clone());
        Self::get_duplicate_identifier_error(fatal, duplicated_id, err_loc, info_loc)
    }

    pub fn duplicate_identifier_error(
        &mut self,
        fatal: bool,
        duplicated_id: String,
        err_line_col: Option<LineColLocation>,
        info_line_col: Option<LineColLocation>,
    ) {
        let err =
            Self::get_duplicate_identifier_error(fatal, duplicated_id, err_line_col, info_line_col);
        self.add_error(err);
    }

    pub fn get_type_check_error(
        fatal: bool,
        message: String,
        loc: &Option<LineColLocation>,
    ) -> WhammError {
        let loc = loc.as_ref().map(|loc| CodeLocation {
            is_err: false,
            message: Some(message.clone()),
            line_col: loc.clone(),
            line_str: None,
            line2_str: None,
        });

        WhammError {
            fatal,
            ty: ErrorType::TypeCheckError {
                message: message.clone(),
            },
            err_loc: loc,
            info_loc: None,
        }
    }

    pub fn get_type_check_error_from_loc(
        fatal: bool,
        message: String,
        line_col: &Option<Location>,
    ) -> WhammError {
        let loc = line_col.as_ref().map(|loc| loc.line_col.clone());
        Self::get_type_check_error(fatal, message, &loc)
    }

    pub fn type_check_error(
        &mut self,
        fatal: bool,
        message: String,
        line_col: &Option<LineColLocation>,
    ) {
        let err = Self::get_type_check_error(fatal, message, line_col);
        self.add_error(err);
    }

    pub fn type_check_error_from_loc(
        &mut self,
        fatal: bool,
        message: String,
        loc: &Option<Location>,
    ) {
        let err = Self::get_type_check_error_from_loc(fatal, message, loc);
        self.add_error(err);
    }

    pub fn get_unexpected_error(
        fatal: bool,
        message: Option<String>,
        line_col: Option<LineColLocation>,
    ) -> WhammError {
        if let Some(line_col) = line_col {
            WhammError {
                fatal,
                ty: ErrorType::Error {
                    message: message.clone(),
                },
                err_loc: Some(CodeLocation {
                    is_err: true,
                    message,
                    line_col,
                    line_str: None,
                    line2_str: None,
                }),
                info_loc: None,
            }
        } else {
            WhammError {
                fatal,
                ty: ErrorType::Error { message },
                err_loc: None,
                info_loc: None,
            }
        }
    }

    pub fn unexpected_error(
        &mut self,
        fatal: bool,
        message: Option<String>,
        line_col: Option<LineColLocation>,
    ) {
        let err = Self::get_unexpected_error(fatal, message, line_col);
        self.add_error(err);
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
            lines
                .rfind(|line| line.as_bytes()[0].is_ascii_digit())
                .map(|line| line.to_string())
        } else {
            None
        };

        let error = if let ParsingError {
            positives,
            negatives,
        } = &e.variant
        {
            WhammError {
                fatal: false,
                ty: ErrorType::ParsingError {
                    positives: positives.clone(),
                    negatives: negatives.clone(),
                    message: None,
                },
                err_loc: Some(CodeLocation {
                    is_err: true,
                    message: None,
                    line_col: e.line_col.clone(),
                    line_str: Some(line),
                    line2_str: line2,
                }),
                info_loc: None,
            }
        } else {
            WhammError {
                fatal: false,
                ty: ErrorType::Error { message: None },
                err_loc: Some(CodeLocation {
                    is_err: true,
                    message: None,
                    line_col: e.line_col.clone(),
                    line_str: Some(line),
                    line2_str: line2,
                }),
                info_loc: None,
            }
        };
        self.add_error(error);
    }

    fn inc_errors(&mut self) {
        self.num_errors += 1;
        self.has_errors = true;
        if self.num_errors >= self.max_errors {
            self.too_many = true;
        }
    }
}

pub struct CodeLocation {
    // True if this is an error-causing code location, false if not (just informational)
    pub is_err: bool,
    // The message associated with this location in the source code
    pub message: Option<String>,
    // The line/column in the source code
    pub line_col: LineColLocation,
    // The line in the source code containing the error
    pub line_str: Option<String>,
    // Possibly a second line if the line_col spans multiple lines
    pub line2_str: Option<String>,
}
impl CodeLocation {
    pub fn is_span(&self) -> bool {
        matches!(self.line_col, LineColLocation::Span(..))
    }
    pub fn lines_are_defined(&self) -> bool {
        self.line_str.is_some()
    }

    // report this error to the console, including color highlighting
    pub fn print(&mut self, script: &str, spacing: &String, buffer: &mut Buffer) {
        if !self.lines_are_defined() {
            self.define_lines(script);
        }

        if let Some(line) = &self.line_str {
            // define common vars for printing
            let (ls, col) = self.start();
            let underline = self.underline(col);
            let message = if let Some(msg) = &self.message {
                msg.clone()
            } else {
                "".to_string()
            };
            if let (LineColLocation::Span(_, (le, _)), Some(ref line2)) =
                (&self.line_col, &self.line2_str)
            {
                let has_line_gap = le - ls > 1;

                if has_line_gap {
                    self.print_numbered_line(ls, line, spacing, buffer);
                    self.print_norm("...", spacing, buffer);
                    self.print_numbered_line(le, line2, spacing, buffer);
                } else {
                    self.print_numbered_line(ls, line, spacing, buffer);
                    self.print_numbered_line(le, line2, spacing, buffer);
                }
            } else {
                self.print_numbered_line(ls, line, spacing, buffer);
            };

            if self.is_err {
                self.print_err(&format!("{underline} {message}"), spacing, buffer);
            } else {
                self.print_info(&format!("{underline} {message}"), spacing, buffer);
            }
        }
    }

    fn define_lines(&mut self, script: &str) {
        match &self.line_col {
            LineColLocation::Pos((line_no, ..)) => {
                if let Some(script_line) = script.lines().nth(line_no - 1) {
                    self.line_str = Some(script_line.to_string());
                }
            }
            LineColLocation::Span((s0_line, ..), (s1_line, ..)) => {
                if let Some(script_line) = script.lines().nth(s0_line - 1) {
                    self.line_str = Some(script_line.to_string());
                }
                if s0_line != s1_line {
                    if let Some(script_line) = script.lines().nth(s1_line - 1) {
                        self.line2_str = Some(script_line.to_string());
                    }
                }
            }
        }
    }

    fn print_numbered_line(&self, l: &usize, line: &String, s: &str, buffer: &mut Buffer) {
        let w = s.len();
        blue(false, format!("{l:w$} | "), buffer);
        white(false, format!("{line}\n"), buffer);
    }

    fn print_line_start(&self, s: &String, buffer: &mut Buffer) {
        blue(false, format!("{s} | "), buffer);
    }

    fn print_err(&self, line: &str, s: &String, buffer: &mut Buffer) {
        self.print_line_start(s, buffer);
        red(false, format!("{line}\n"), buffer);
    }

    fn print_info(&self, line: &str, s: &String, buffer: &mut Buffer) {
        self.print_line_start(s, buffer);
        blue(false, format!("{line}\n"), buffer);
    }

    fn print_norm(&self, line: &str, s: &String, buffer: &mut Buffer) {
        self.print_line_start(s, buffer);
        white(false, format!("{line}\n"), buffer);
    }

    fn underline(&self, start_col: &usize) -> String {
        let mut underline = String::new();

        let mut start_col = *start_col;
        let end = match &self.line_col {
            LineColLocation::Span(_, (_, mut end)) => {
                let inverted_cols = start_col > end;
                if inverted_cols {
                    mem::swap(&mut start_col, &mut end);
                    start_col -= 1;
                    end += 1;
                }

                Some(end)
            }
            _ => None,
        };
        let offset = start_col - 1;

        if let Some(line) = &self.line_str {
            let line_chars = line.chars();

            for c in line_chars.take(offset) {
                match c {
                    '\t' => underline.push('\t'),
                    _ => underline.push(' '),
                }
            }
        }

        if let Some(end) = end {
            let u_char = if self.is_err {
                ERR_UNDERLINE_CHAR
            } else {
                INFO_UNDERLINE_CHAR
            };

            underline.push(u_char);
            if end - start_col > 1 {
                for _ in 2..(end - start_col) {
                    underline.push(u_char);
                }
                underline.push(u_char);
            }
        } else {
            underline.push_str("^---")
        }

        underline
    }

    fn start(&self) -> &(usize, usize) {
        match &self.line_col {
            LineColLocation::Pos(line_col) => line_col,
            LineColLocation::Span(start_line_col, _) => start_line_col,
        }
    }
}

pub struct WhammError {
    pub fatal: bool,
    /// The location within the input string causing the error
    pub err_loc: Option<CodeLocation>,
    /// A location within the input string that can add context to the error
    pub info_loc: Option<CodeLocation>,
    pub ty: ErrorType,
}
impl WhammError {
    pub fn is_fatal(&self) -> bool {
        self.fatal
    }

    /// report this error to the console, including color highlighting
    pub fn report(&mut self, script: &str, script_path: &String) {
        let spacing = self.spacing();
        let message = self.ty.message();

        let writer = BufferWriter::stderr(ColorChoice::Always);
        let mut buffer = writer.buffer();

        red(true, format!("error[{}]", self.ty.name()), &mut buffer);
        white(true, format!(": {}\n", message), &mut buffer);

        if let Some(err_loc) = &mut self.err_loc {
            if err_loc.message.is_none() {
                err_loc.message = Some(message.clone().to_string());
            }

            print_preamble(&err_loc.line_col, script_path, &spacing, &mut buffer);
            print_empty(&spacing, &mut buffer);
            let err_start = match &err_loc.line_col {
                LineColLocation::Pos((line, _)) => line,
                LineColLocation::Span((start_line, _), ..) => start_line,
            };
            if let Some(info_loc) = &mut self.info_loc {
                let info_start = match &info_loc.line_col {
                    LineColLocation::Pos((line, _)) => line,
                    LineColLocation::Span((start_line, _), ..) => start_line,
                };

                if info_start < err_start {
                    // print info first
                    info_loc.print(script, &spacing, &mut buffer);
                    err_loc.print(script, &spacing, &mut buffer);
                } else {
                    // print err first
                    err_loc.print(script, &spacing, &mut buffer);
                    info_loc.print(script, &spacing, &mut buffer);
                }
            } else {
                // only print err
                err_loc.print(script, &spacing, &mut buffer);
            }
            print_empty(&spacing, &mut buffer);
        } else {
            // This error isn't tied to a specific code location
            blue(false, " --> ".to_string(), &mut buffer);
            blue(false, format!("{script_path}\n\n"), &mut buffer);
        }
        writer
            .print(&buffer)
            .expect("Uh oh, something went wrong while printing to terminal");
        buffer
            .reset()
            .expect("Uh oh, something went wrong while printing to terminal");
    }

    fn spacing(&self) -> String {
        let largest_err_line_no = if let Some(err_loc) = &self.err_loc {
            match &err_loc.line_col {
                LineColLocation::Pos((line, _)) => line,
                LineColLocation::Span((start_line, _), (end_line, _)) => {
                    cmp::max(start_line, end_line)
                }
            }
        } else {
            // No err_line, return empty string
            return "".to_string();
        };
        let largest_info_line_no = if let Some(info_loc) = &self.info_loc {
            match &info_loc.line_col {
                LineColLocation::Pos((line, _)) => line,
                LineColLocation::Span((start_line, _), (end_line, _)) => {
                    cmp::max(start_line, end_line)
                }
            }
        } else {
            // Assuming if we get here, there IS an err_line_no set; just
            // return a "short" number
            &0
        };
        let largest_line_no = cmp::max(largest_err_line_no, largest_info_line_no);

        // calculate the length of the longest line number (in chars)
        let line_str_len = format!("{}", largest_line_no).len();

        let mut spacing = String::new();
        for _ in 0..line_str_len {
            spacing.push(' ');
        }

        spacing
    }
}

pub enum ErrorType {
    DuplicateIdentifierError {
        duplicated_id: String,
    },
    /// Generated parsing error with expected and unexpected `Rule`s
    ParsingError {
        /// Positive attempts
        positives: Vec<Rule>,
        /// Negative attempts
        negatives: Vec<Rule>,
        message: Option<String>,
    },
    /// Error during type checking
    TypeCheckError {
        message: String,
    },
    Error {
        message: Option<String>,
    },
}
impl ErrorType {
    pub fn name(&self) -> &str {
        match self {
            ErrorType::DuplicateIdentifierError { .. } => "DuplicateIdentifierError",
            ErrorType::ParsingError { .. } => "ParsingError",
            ErrorType::TypeCheckError { .. } => "TypeCheckError",
            ErrorType::Error { .. } => "GeneralError",
        }
    }
    pub fn message(&self) -> Cow<'_, str> {
        match self {
            ErrorType::ParsingError {
                ref positives,
                ref negatives,
                ref message,
            } => Cow::Owned(Self::parsing_error_message(
                message,
                positives,
                negatives,
                |r| format!("{:?}", r),
            )),
            ErrorType::TypeCheckError { ref message } => Cow::Borrowed(message),
            ErrorType::DuplicateIdentifierError { ref duplicated_id } => {
                Cow::Owned(format!("duplicate definitions with name `{duplicated_id}`"))
            }
            ErrorType::Error { ref message } => {
                if let Some(msg) = message {
                    Cow::Borrowed(msg)
                } else {
                    Cow::Borrowed("An error occurred.")
                }
            }
        }
    }

    fn parsing_error_message<F>(
        message: &Option<String>,
        positives: &[Rule],
        negatives: &[Rule],
        mut f: F,
    ) -> String
    where
        F: FnMut(&Rule) -> String,
    {
        let preamble = if let Some(msg) = message {
            let mut s = msg.to_string();
            if !negatives.is_empty() && !positives.is_empty() {
                s += " -- ";
            }
            s
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
            (false, true) => format!(
                "{}unexpected {}",
                preamble,
                ErrorType::enumerate(negatives, &mut f)
            ),
            (true, false) => format!(
                "{}expected {}",
                preamble,
                ErrorType::enumerate(positives, &mut f)
            ),
            (true, true) => {
                if preamble.is_empty() {
                    "unknown parsing error".to_owned()
                } else {
                    preamble
                }
            }
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

fn print_preamble(
    line_col: &LineColLocation,
    script_path: &String,
    s: &String,
    buffer: &mut Buffer,
) {
    let (ls, c) = match line_col {
        LineColLocation::Pos(line_col) => line_col,
        LineColLocation::Span(start_line_col, _) => start_line_col,
    };

    blue(false, format!("{s}--> "), buffer);
    blue(false, format!("{script_path}:"), buffer);
    blue(false, format!("{ls}:{c}\n"), buffer);
}

fn print_line(line: &str, is_err: bool, s: &String, buffer: &mut Buffer) {
    blue(false, format!("{s} | "), buffer);
    if is_err {
        red(false, format!("{line}\n"), buffer);
    } else {
        white(false, format!("{line}\n"), buffer);
    }
}

fn print_empty(s: &String, buffer: &mut Buffer) {
    print_line("", false, s, buffer);
}
