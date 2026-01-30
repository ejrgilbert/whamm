use crate::common::terminal::{blue, red, white, yellow};
use crate::parser::types::{Location, Rule};
use pest::error::ErrorVariant::ParsingError;
use pest::error::{Error, LineColLocation};
use std::borrow::Cow;
use std::{cmp, mem};
use termcolor::{Buffer, BufferWriter, ColorChoice, WriteColor};

const ERR_UNDERLINE_CHAR: char = '^';
const INFO_UNDERLINE_CHAR: char = '-';

pub struct ErrorGen {
    curr_match_rule: Option<String>,
    script_path: String,
    script_text: String,
    max_errors: i32,
    errors: Vec<WhammError>,
    warnings: Vec<WhammWarning>,
    num_errors: i32,
    pub too_many: bool,
    pub has_errors: bool,
    pub has_warnings: bool,
}
impl ErrorGen {
    pub fn new(script_path: String, script_text: String, max_errors: i32) -> Self {
        Self {
            curr_match_rule: None,
            script_path,
            script_text,
            max_errors,
            errors: vec![],
            warnings: vec![],
            num_errors: 0,
            too_many: false,
            has_errors: false,
            has_warnings: false,
        }
    }

    pub fn pull_errs(&self) -> Vec<WhammError> {
        self.errors.to_owned()
    }

    pub fn update_match_rule(&mut self, match_rule: Option<String>) {
        self.curr_match_rule = match_rule;
    }

    pub fn add_error(&mut self, mut error: WhammError) {
        error.match_rule = self.curr_match_rule.clone();
        self.errors.push(error);
        self.inc_errors();
    }

    pub fn add_errors(&mut self, errors: Vec<WhammError>) -> Result<(), ()> {
        for error in errors {
            self.add_error(error);
        }

        if self.too_many { Err(()) } else { Ok(()) }
    }

    pub fn set_script_text(&mut self, script_text: String) {
        self.script_text = script_text;
    }

    pub fn report_warnings(&mut self) {
        self.warnings.iter_mut().for_each(|warning| {
            warning.report(&self.script_text, &self.script_path);
        });
        self.warnings.clear();
    }

    pub fn report(&mut self) {
        self.report_warnings();
        // Report the most-recent error first
        self.errors.iter_mut().for_each(|error| {
            error.report(&self.script_text, &self.script_path);
        });
        self.errors.clear();
    }

    // ======================
    // == Error Generators ==
    // ======================

    fn get_error_loc(message: &str, loc: &Option<Location>) -> Option<CodeLocation> {
        loc.as_ref().map(|err_loc| CodeLocation {
            ty: LocType::Err,
            message: Some(message.to_string()),
            line_col: err_loc.line_col.clone(),
            line_str: None,
            line2_str: None,
        })
    }

    fn get_warn_loc(message: &str, loc: &Option<Location>) -> Option<CodeLocation> {
        loc.as_ref().map(|err_loc| CodeLocation {
            ty: LocType::Warn,
            message: Some(message.to_string()),
            line_col: err_loc.line_col.clone(),
            line_str: None,
            line2_str: None,
        })
    }

    pub fn get_unimplemented_error(message: &str, loc: &Option<Location>) -> WhammError {
        WhammError {
            match_rule: None,
            err_loc: Self::get_error_loc(message, loc),
            ty: ErrorType::UnimplementedError {
                message: message.to_string(),
            },
            info_loc: None,
        }
    }
    pub fn add_unimplemented_error(&mut self, msg: &str, loc: &Option<Location>) {
        self.add_error(Self::get_unimplemented_error(msg, loc));
    }

    pub fn get_internal_error(message: &str, loc: &Option<Location>) -> WhammError {
        WhammError {
            match_rule: None,
            err_loc: Self::get_error_loc(message, loc),
            ty: ErrorType::InternalError {
                message: message.to_string(),
            },
            info_loc: None,
        }
    }
    pub fn add_internal_error(&mut self, msg: &str, loc: &Option<Location>) {
        self.add_error(Self::get_internal_error(msg, loc));
    }

    pub fn get_instrumentation_error(message: &str) -> WhammError {
        WhammError {
            match_rule: None,
            ty: ErrorType::InstrumentationError {
                message: message.to_string(),
            },
            err_loc: None,
            info_loc: None,
        }
    }
    pub fn add_instr_error(&mut self, msg: &str) {
        self.add_error(Self::get_instrumentation_error(msg));
    }

    pub fn multiple_alt_matches(&mut self, instr_name: &str) {
        let msg = &format!(
            "Multiple `alt` probes matched same bytecode location for instr_name: {}",
            instr_name
        );
        self.add_error(Self::get_instrumentation_error(msg));
    }

    pub fn get_arithmetic_error(message: &str, loc: Option<Location>) -> WhammError {
        WhammError {
            match_rule: None,
            err_loc: Self::get_error_loc(message, &loc),
            ty: ErrorType::ArithmeticError {
                message: message.to_string(),
            },
            info_loc: None,
        }
    }

    pub fn div_by_zero(&mut self, loc: Option<Location>) {
        let err = Self::get_arithmetic_error("attempt to divide by zero", loc);
        self.add_error(err);
    }

    pub fn get_parse_error(
        message: Option<String>,
        line_col: Option<LineColLocation>,
        positives: Vec<Rule>,
        negatives: Vec<Rule>,
    ) -> WhammError {
        if let Some(line_col) = line_col {
            WhammError {
                match_rule: None,
                ty: ErrorType::ParsingError {
                    positives,
                    negatives,
                    message: message.clone(),
                },
                err_loc: Some(CodeLocation {
                    ty: LocType::Err,
                    message,
                    line_col,
                    line_str: None,
                    line2_str: None,
                }),
                info_loc: None,
            }
        } else {
            WhammError {
                match_rule: None,
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
    pub fn parse_error_at_loc(&mut self, message: Option<String>, line_col: Option<Location>) {
        let err_loc = line_col.as_ref().map(|err_loc| err_loc.line_col.clone());
        let err = Self::get_parse_error(message, err_loc, vec![], vec![]);
        self.add_error(err);
    }

    pub fn parse_error(
        &mut self,
        message: Option<String>,
        line_col: Option<LineColLocation>,
        positives: Vec<Rule>,
        negatives: Vec<Rule>,
    ) {
        let err = Self::get_parse_error(message, line_col, positives, negatives);
        self.add_error(err);
    }

    pub fn get_duplicate_identifier_error(
        duplicated_id: String,
        err_line_col: Option<LineColLocation>,
        info_line_col: Option<LineColLocation>,
    ) -> WhammError {
        let err_loc = err_line_col.map(|err_line_col| CodeLocation {
            ty: LocType::Err,
            message: Some(format!("duplicate definitions for `{}`", duplicated_id)),
            line_col: err_line_col,
            line_str: None,
            line2_str: None,
        });
        let info_loc = info_line_col.map(|info_line_col| CodeLocation {
            ty: LocType::Info,
            message: Some(format!("other definition for `{}`", duplicated_id)),
            line_col: info_line_col,
            line_str: None,
            line2_str: None,
        });

        WhammError {
            match_rule: None,
            ty: ErrorType::DuplicateIdentifierError {
                duplicated_id: duplicated_id.clone(),
            },
            err_loc,
            info_loc,
        }
    }
    pub fn get_compiler_fn_overload_error(
        duplicated_id: String,
        loc: Option<LineColLocation>,
    ) -> WhammError {
        let err_loc = loc.map(|err_line_col| CodeLocation {
            ty: LocType::Err,
            message: Some(format!(
                "`{}` is an identifier used by compiler. Neither overloading nor overriding is supported",
                duplicated_id
            )),
            line_col: err_line_col,
            line_str: None,
            line2_str: None,
        });

        WhammError {
            match_rule: None,
            ty: ErrorType::DuplicateIdentifierError {
                duplicated_id: duplicated_id.clone(),
            },
            err_loc,
            info_loc: None,
        }
    }
    pub fn compiler_fn_overload_error(
        &mut self,
        duplicated_id: String,
        loc: Option<LineColLocation>,
    ) {
        let err = Self::get_compiler_fn_overload_error(duplicated_id, loc);
        self.add_error(err);
    }
    pub fn duplicate_identifier_error(
        &mut self,
        duplicated_id: String,
        err_line_col: Option<LineColLocation>,
        info_line_col: Option<LineColLocation>,
    ) {
        let err = Self::get_duplicate_identifier_error(duplicated_id, err_line_col, info_line_col);
        self.add_error(err);
    }
    pub fn get_type_check_error(message: String, loc: &Option<LineColLocation>) -> WhammError {
        let loc = loc.as_ref().map(|loc| CodeLocation {
            ty: LocType::Err,
            message: Some(message.clone()),
            line_col: loc.clone(),
            line_str: None,
            line2_str: None,
        });

        WhammError {
            match_rule: None,
            ty: ErrorType::TypeCheckError {
                message: message.clone(),
            },
            err_loc: loc,
            info_loc: None,
        }
    }

    pub fn type_check_error(&mut self, message: String, line_col: &Option<LineColLocation>) {
        let err = Self::get_type_check_error(message, line_col);
        self.add_error(err);
    }
    pub fn get_wei_error(message: String, loc: &Option<LineColLocation>) -> WhammError {
        let loc = loc.as_ref().map(|loc| CodeLocation {
            ty: LocType::Err,
            message: Some(message.clone()),
            line_col: loc.clone(),
            line_str: None,
            line2_str: None,
        });

        WhammError {
            match_rule: None,
            ty: ErrorType::WeiError {
                message: message.clone(),
            },
            err_loc: loc,
            info_loc: None,
        }
    }

    pub fn get_wei_error_from_loc(message: String, line_col: &Option<Location>) -> WhammError {
        let loc = line_col.as_ref().map(|loc| loc.line_col.clone());
        Self::get_wei_error(message, &loc)
    }

    pub fn wei_error(&mut self, message: String, loc: &Option<Location>) {
        let err = Self::get_wei_error_from_loc(message, loc);
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
                match_rule: self.curr_match_rule.clone(),
                ty: ErrorType::ParsingError {
                    positives: positives.clone(),
                    negatives: negatives.clone(),
                    message: None,
                },
                err_loc: Some(CodeLocation {
                    ty: LocType::Err,
                    message: None,
                    line_col: e.line_col.clone(),
                    line_str: Some(line),
                    line2_str: line2,
                }),
                info_loc: None,
            }
        } else {
            WhammError {
                match_rule: self.curr_match_rule.clone(),
                ty: ErrorType::Error { message: None },
                err_loc: Some(CodeLocation {
                    ty: LocType::Err,
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

    // ==================
    // ==== WARNINGS ====
    // ==================

    pub fn add_warn(&mut self, warn: WhammWarning) {
        self.warnings.push(warn);
        self.has_warnings = true;
    }
    pub fn get_probe_warning(message: &str, loc: &Option<Location>) -> WhammWarning {
        WhammWarning {
            match_rule: None,
            ty: WarnType::ProbeWarning {
                message: message.to_string(),
            },
            warn_loc: Self::get_warn_loc(message, loc),
            info_loc: None,
        }
    }
    pub fn add_probe_warn(&mut self, message: &str, loc: &Option<Location>) {
        self.add_warn(Self::get_probe_warning(message, loc));
    }
    pub fn add_typecheck_warn(&mut self, message: String, loc: Option<LineColLocation>) {
        let loc = loc.as_ref().map(|loc| CodeLocation {
            ty: LocType::Warn,
            message: Some(message.clone()),
            line_col: loc.clone(),
            line_str: None,
            line2_str: None,
        });
        let warn = WhammWarning {
            match_rule: self.curr_match_rule.clone(),
            ty: WarnType::TypeCheckWarning { message },
            warn_loc: loc,
            info_loc: None,
        };
        self.add_warn(warn);
    }
}

#[derive(Clone, Debug)]
enum LocType {
    /// Is an error-causing code location
    Err,
    /// Is a warning-causing code location
    Warn,
    /// Is just informational
    Info,
}
#[derive(Clone, Debug)]
pub struct CodeLocation {
    ty: LocType,
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
    pub fn print(&mut self, script: &str, spacing: &str, buffer: &mut Buffer) {
        if !self.lines_are_defined() {
            self.define_lines(script);
        }

        if let Some(line) = &self.line_str {
            // define common vars for printing
            let (ls, _) = self.start();
            if let (LineColLocation::Span(_, (le, _)), Some(line2)) =
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

            self.print_underline(spacing, buffer);
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

    fn print_line_start(&self, s: &str, buffer: &mut Buffer) {
        blue(false, format!("{s} | "), buffer);
    }

    fn print_underline(&self, s: &str, buffer: &mut Buffer) {
        let (_, col) = self.start();
        let underline = self.underline(col);
        let message = if let Some(msg) = &self.message {
            msg.clone()
        } else {
            "".to_string()
        };

        self.print_line_start(s, buffer);
        let color = match self.ty {
            LocType::Err => red,
            LocType::Warn => yellow,
            LocType::Info => blue,
        };
        color(false, format!("{underline} {message}\n"), buffer);
    }

    fn print_norm(&self, line: &str, s: &str, buffer: &mut Buffer) {
        self.print_line_start(s, buffer);
        white(false, format!("{line}\n"), buffer);
    }

    fn underline(&self, start_col: &usize) -> String {
        let mut underline = String::new();

        let mut start_col = *start_col;
        let end = match &self.line_col {
            LineColLocation::Span(_, (_, end)) => {
                let mut end_inner = *end;
                let inverted_cols = start_col > end_inner;
                if inverted_cols {
                    mem::swap(&mut start_col, &mut end_inner.clone());
                    start_col -= 1;
                    end_inner += 1;
                }

                Some(end_inner)
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
            let u_char = match self.ty {
                LocType::Err => ERR_UNDERLINE_CHAR,
                LocType::Warn | LocType::Info => INFO_UNDERLINE_CHAR,
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
#[derive(Clone, Debug)]
pub struct WhammError {
    pub match_rule: Option<String>,
    /// The location within the input string causing the error
    pub err_loc: Option<CodeLocation>,
    /// A location within the input string that can add context to the error
    pub info_loc: Option<CodeLocation>,
    pub ty: ErrorType,
}

impl From<std::io::Error> for Box<WhammError> {
    fn from(e: std::io::Error) -> Self {
        Box::new(WhammError {
            match_rule: None,
            err_loc: None,
            info_loc: None,
            ty: ErrorType::Error {
                message: Some(e.to_string()),
            },
        })
    }
}

pub struct WhammWarning {
    pub match_rule: Option<String>,
    pub ty: WarnType,
    pub warn_loc: Option<CodeLocation>,
    pub info_loc: Option<CodeLocation>,
}
impl WhammWarning {
    pub fn report(&mut self, script: &str, script_path: &String) {
        let spacing = self.spacing();
        let message = self.ty.message();

        let writer = BufferWriter::stderr(ColorChoice::Always);
        let mut buffer = writer.buffer();

        let preamble = if let Some(rule) = &self.match_rule {
            format!("warning[{}]@{rule}", self.ty.name())
        } else {
            format!("warning[{}]", self.ty.name())
        };
        yellow(true, preamble, &mut buffer);
        white(true, format!(": {}\n", message), &mut buffer);

        if let Some(warn_loc) = &mut self.warn_loc {
            if warn_loc.message.is_none() {
                warn_loc.message = Some(message.clone().to_string());
            }

            print_preamble(&warn_loc.line_col, script_path, &spacing, &mut buffer);
            print_empty(&spacing, &mut buffer);
            let warn_start = match &warn_loc.line_col {
                LineColLocation::Pos((line, _)) => line,
                LineColLocation::Span((start_line, _), ..) => start_line,
            };
            if let Some(info_loc) = &mut self.info_loc {
                let info_start = match &info_loc.line_col {
                    LineColLocation::Pos((line, _)) => line,
                    LineColLocation::Span((start_line, _), ..) => start_line,
                };

                if info_start < warn_start {
                    // print info first
                    info_loc.print(script, &spacing, &mut buffer);
                    warn_loc.print(script, &spacing, &mut buffer);
                } else {
                    // print err first
                    warn_loc.print(script, &spacing, &mut buffer);
                    info_loc.print(script, &spacing, &mut buffer);
                }
            } else {
                // only print err
                warn_loc.print(script, &spacing, &mut buffer);
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
        let largest_err_line_no = if let Some(warn_loc) = &self.warn_loc {
            match &warn_loc.line_col {
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
impl WhammError {
    /// report this error to the console, including color highlighting
    pub fn report(&mut self, script: &str, script_path: &String) {
        let spacing = self.spacing();
        let message = self.ty.message();

        let writer = BufferWriter::stderr(ColorChoice::Always);
        let mut buffer = writer.buffer();

        let preamble = if let Some(rule) = &self.match_rule {
            format!("error[{}]@{rule}", self.ty.name())
        } else {
            format!("error[{}]", self.ty.name())
        };
        red(true, preamble, &mut buffer);
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
pub enum WarnType {
    ProbeWarning { message: String },
    TypeCheckWarning { message: String },
}
impl WarnType {
    pub fn name(&self) -> &str {
        match self {
            WarnType::ProbeWarning { .. } => "ProbeWarning",
            WarnType::TypeCheckWarning { .. } => "TypeCheckWarning",
        }
    }
    pub fn message(&self) -> Cow<'_, str> {
        match self {
            WarnType::ProbeWarning { message } | WarnType::TypeCheckWarning { message } => {
                Cow::Borrowed(message)
            }
        }
    }
}
#[derive(Clone, Debug)]
pub enum ErrorType {
    UnimplementedError {
        message: String,
    },
    InternalError {
        message: String,
    },
    InstrumentationError {
        message: String,
    },
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
    /// Error when compiling to wei target
    WeiError {
        message: String,
    },
    Error {
        message: Option<String>,
    },
    ArithmeticError {
        message: String,
    },
}
impl ErrorType {
    pub fn name(&self) -> &str {
        match self {
            ErrorType::UnimplementedError { .. } => "Unimplemented",
            ErrorType::InternalError { .. } => "InternalError",
            ErrorType::InstrumentationError { .. } => "InstrumentationError",
            ErrorType::DuplicateIdentifierError { .. } => "DuplicateIdentifierError",
            ErrorType::ParsingError { .. } => "ParsingError",
            ErrorType::TypeCheckError { .. } => "TypeCheckError",
            ErrorType::WeiError { .. } => "WeiError",
            ErrorType::Error { .. } => "GeneralError",
            ErrorType::ArithmeticError { .. } => "ArithmeticError",
        }
    }
    pub fn message(&self) -> Cow<'_, str> {
        match self {
            ErrorType::UnimplementedError { message }
            | ErrorType::InternalError { message }
            | ErrorType::ArithmeticError { message }
            | ErrorType::InstrumentationError { message } => Cow::Borrowed(message),
            ErrorType::ParsingError {
                positives,
                negatives,
                message,
            } => Cow::Owned(Self::parsing_error_message(
                message,
                positives,
                negatives,
                |r| format!("{:?}", r),
            )),
            ErrorType::TypeCheckError { message } | ErrorType::WeiError { message } => {
                Cow::Borrowed(message)
            }
            ErrorType::DuplicateIdentifierError { duplicated_id } => {
                Cow::Owned(format!("duplicate definitions with name `{duplicated_id}`"))
            }
            ErrorType::Error { message } => {
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
