use std::{cmp, mem};
use std::borrow::Cow;
use std::io::Write;
use pest::error::{Error, LineColLocation};
use pest::RuleType;
use pest::error::ErrorVariant::ParsingError;
use termcolor::{Buffer, BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};

// struct ErrorGen {
//     errors: Vec<Error>,
//     num_errors: i32,
//     not_too_many: bool,
//     no_errors: bool
// }
// impl ErrorGen {
//     fn report(&mut self) {
//         self.errors.iter().for_each(|error| {
//             error.report();
//         });
//         self.errors.clear();
//     }
//     fn fatal_report(&mut self, context: String) {
//         if self.no_errors {
//             return;
//         }
//         self.report();
//         System.error(context, "expected no errors");
//     }
//     // fn copy(&that: ErrorGen) {
//     // this.errors = that.errors;
//     // this.noErrors = that.noErrors;
//     // this.numErrors = that.numErrors;
//     // this.notTooMany = that.notTooMany;
//     // }
//     fn firstError(&self) -> String {
//         if let Some(first) = self.errors.first() {
//             return first.to_string();
//         }
//         return "".to_string();
//     }
//     def OutputError(fileName: string) {
//     addError(null, null, "Cannot create output file", fileName);
//     }
//     def ExceptionInInitialization(meth: VstMethod, except: Exception) {
//     var msg = TerminalBuffer.new().putv(except, null).toString();
//     addError(meth.token.range(), null, "ExceptionInInitialization", msg);
//     }
//     def FileNotFound(fname: string) {
//     addError(null, null, "File not found", fname);
//     }
//     def MainNotFound() {
//     addError(null, null, null, "Main not found");
//     }
//     def addError(range: FileRange, sourceLine: string, error: string, msg: string) {
//     errors = List.new(Error.new(range, sourceLine, error, msg), errors);
//     incErrors();
//     }
//     def unexpectedType<T>(t: Type, v: T) -> T {
//     var msg = Strings.format1("unexpected type %q", t.render);
//     if (CLOptions.FATAL.get()) return V3.fail(msg);
//     addError(null, null, null, msg);
//     return v;
// }
// def incErrors() {
// numErrors++;
// noErrors = false;
// if (numErrors >= maxErrors) notTooMany = false;
// }
// }

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

pub struct WhammError<R> {
    pub fatal: bool,
    /// Line/column within the input string
    pub line_col: LineColLocation,
    pub path: Option<String>,
    // range: FileRange,
    pub ty: ErrorType<R>,
    // /// Location within the input string
    // pub location: InputLocation,
    pub line: Option<String>,
    pub line2: Option<String>,
    // error: String,
}
impl<R: RuleType> WhammError<R> {
    pub fn parse_error(fatal: bool, message: Option<String>, line_col: LineColLocation,
                       positives: Vec<R>, negatives: Vec<R>,
                       whammy_path: Option<String>) -> Self {
        // TODO -- move to ErrorGen and exit if fatal (can also save whammy_path as field)
        WhammError {
            fatal,
            ty: ErrorType::ParsingError {
                positives,
                negatives,
                message
            },
            line_col,
            path: whammy_path,
            line: None,
            line2: None
        }
    }

    pub fn type_check_error(fatal: bool, message: String, line_col: LineColLocation,
                            whammy_path: Option<String>) -> Self {
        // TODO -- move to ErrorGen and exit if fatal (can also save whammy_path as field)
        WhammError {
            fatal,
            ty: ErrorType::TypeCheckError {
                message
            },
            line_col,
            path: whammy_path,
            line: None,
            line2: None
        }
    }

    pub fn from_pest_err(e: Error<R>, whammy_path: &String) -> Self {
        let path = if let Some(p) = e.path() {
            Some(p.to_string())
        } else {
            Some(whammy_path.clone())
        };
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

        if let ParsingError {positives, negatives} = &e.variant {
            WhammError {
                fatal: false,
                ty: ErrorType::ParsingError {
                    positives: positives.clone(),
                    negatives: negatives.clone(),
                    message: None
                },
                line_col: e.line_col.clone(),
                path,
                line: Some(line),
                line2
            }
        } else {
            // TODO error -- unsupported Pest error
            todo!()
        }
    }

    fn define_lines(&mut self, script: &String) {
        match self.line_col {
            LineColLocation::Pos((line_no, ..)) => {
                if let Some(script_line) = script.lines().nth(line_no) {
                    self.line = Some(script_line.to_string());
                }
            }
            LineColLocation::Span((s0_line, ..), (s1_line, ..)) => {
                if let Some(script_line) = script.lines().nth(s0_line) {
                    self.line = Some(script_line.to_string());
                }
                if let Some(script_line) = script.lines().nth(s1_line) {
                    self.line = Some(script_line.to_string());
                }
            }
        }
    }

    fn print_preamble(&self, extra_spaces: usize, buffer: &mut Buffer) {
        let s = self.spacing();
        let ls = self.start().0;
        let c = self.start().1;
        let spaces = " ".repeat(extra_spaces);

        blue(format!("{s}{spaces}--> "), buffer);
        if let Some(path) = &self.path {
            blue(format!("{path}:"), buffer);
        }
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

    // report this error to the console, including color highlighting
    pub fn report(&mut self, script: &String) {
        if self.line.is_none() {
            self.define_lines(script);
        }

        if let Some(line) = &self.line {
            let pair = (self.line_col.clone(), &self.line2);
            let writer = BufferWriter::stderr(ColorChoice::Always);
            let mut buffer = writer.buffer();

            // define common vars for printing
            let ls = self.start().0;
            let underline = self.underline();
            let message = self.ty.message();
            if let (LineColLocation::Span(_, end), Some(ref line2)) = pair {
                let has_line_gap = end.0 - self.start().0 > 1;

                // define common vars for printing
                let le = end.0;
                if has_line_gap {
                    self.print_preamble(4, &mut buffer);
                    self.print_empty(4, &mut buffer);
                    self.print_numbered_line(ls, line, &mut buffer);
                    self.print_line("...", false, 4, &mut buffer);
                    self.print_numbered_line(le, line2, &mut buffer);
                    self.print_line(&format!("{underline} {message}"), true, 4, &mut buffer);
                    self.print_empty(4, &mut buffer);
                } else {
                    self.print_preamble(4, &mut buffer);
                    self.print_empty(4, &mut buffer);
                    self.print_numbered_line(ls, line, &mut buffer);
                    self.print_numbered_line(le, line2, &mut buffer);
                    self.print_line(&format!("{underline} {message}"), true, 4, &mut buffer);
                    self.print_empty(4, &mut buffer);
                }
            } else {
                self.print_preamble(0, &mut buffer);
                self.print_empty(0, &mut buffer);
                self.print_numbered_line(ls, line, &mut buffer);
                self.print_line(&format!("{underline} {message}"), true, 0, &mut buffer);
                self.print_empty(0, &mut buffer);
            };
            writer.print(&buffer).expect("uh oh");
            buffer.reset().expect("uh oh");
        } else {
            // TODO -- report issue
        }
    }

    fn spacing(&self) -> String {
        let line = match self.line_col {
            LineColLocation::Pos((line, _)) => line,
            LineColLocation::Span((start_line, _), (end_line, _)) => {
                cmp::max(start_line, end_line)
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
            LineColLocation::Span(_, (_, mut end)) => {
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
            LineColLocation::Pos(line_col) => line_col,
            LineColLocation::Span(start_line_col, _) => start_line_col,
        }
    }
}

pub enum ErrorType<R> {
    /// Generated parsing error with expected and unexpected `Rule`s
    ParsingError {
        /// Positive attempts
        positives: Vec<R>,
        /// Negative attempts
        negatives: Vec<R>,
        message: Option<String>
    },
    /// Error during type checking
    TypeCheckError {
        message: String
    }
}
impl<R: RuleType> ErrorType<R> {
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
        }
    }

    fn parsing_error_message<F>(message: &Option<String>, positives: &[R], negatives: &[R], mut f: F) -> String
        where F: FnMut(&R) -> String,
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

    fn enumerate<F>(rules: &[R], f: &mut F) -> String
        where
            F: FnMut(&R) -> String,
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