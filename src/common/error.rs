use std::{cmp, mem};
use std::io::Write;
use pest::error::{Error, ErrorVariant, InputLocation, LineColLocation};
use pest::RuleType;
use termcolor::{Buffer, BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};
use crate::parser::types::Rule;

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
// Black,
// Blue,
// Green,
// Red,
// Cyan,
// Magenta,
// Yellow,
// White,

fn color(s: String, buffer: &mut Buffer, c: Color) {
    let write_err = "Uh oh, something went wrong while printing to terminal";
    buffer.set_color(ColorSpec::new().set_fg(Some(c))).expect(write_err);
    write!(buffer, "{}", s.as_str()).expect(write_err);
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
    // range: FileRange,
    pub variant: ErrorVariant<R>,
    // /// Location within the input string
    // pub location: InputLocation,
    /// Line/column within the input string
    pub line_col: LineColLocation,
    pub path: Option<String>,
    pub continued_line: Option<String>,
    pub line: String,
    // error: String,
}
impl<R: RuleType> WhammError<R> {
    pub fn from_pest_err(e: Error<R>, whammy_path: &String) -> Self {
        let path = if let Some(p) = e.path() {
            Some(p.to_string())
        } else {
            Some(whammy_path.clone())
        };
        let line = e.line().to_string();

        WhammError {
            variant: e.variant,
            // location: e.location.clone(),
            line_col: e.line_col.clone(),
            path,
            continued_line: None,
            line
        }
    }

    // report this error to the console, including color highlighting
    pub fn report(&self) {
        let spacing = self.spacing();
        let path = self
            .path
            .as_ref()
            .map(|path| format!("{}:", path))
            .unwrap_or_default();

        let pair = (self.line_col.clone(), &self.continued_line);
        let writer = BufferWriter::stderr(ColorChoice::Always);
        let mut buffer = writer.buffer();
        if let (LineColLocation::Span(_, end), Some(ref continued_line)) = pair {
            let has_line_gap = end.0 - self.start().0 > 1;
            if has_line_gap {
                format!(
                    "{s    }--> {p}{ls}:{c}\n\
                     {s    } |\n\
                     {ls:w$} | {line}\n\
                     {s    } | ...\n\
                     {le:w$} | {continued_line}\n\
                     {s    } | {underline}\n\
                     {s    } |\n\
                     {s    } = {message}",
                    s = spacing,
                    w = spacing.len(),
                    p = path,
                    ls = self.start().0,
                    le = end.0,
                    c = self.start().1,
                    line = self.line(),
                    continued_line = continued_line,
                    underline = self.underline(),
                    message = self.variant.message()
                );
            } else {
                format!(
                    "{s    }--> {p}{ls}:{c}\n\
                     {s    } |\n\
                     {ls:w$} | {line}\n\
                     {le:w$} | {continued_line}\n\
                     {s    } | {underline}\n\
                     {s    } |\n\
                     {s    } = {message}",
                    s = spacing,
                    w = spacing.len(),
                    p = path,
                    ls = self.start().0,
                    le = end.0,
                    c = self.start().1,
                    line = self.line,
                    continued_line = continued_line,
                    underline = self.underline(),
                    message = self.variant.message()
                );
            }
        } else {
            let s = spacing;
            let l = self.start().0;
            let c = self.start().1;
            let line = &self.line;
            let underline = self.underline();
            let message = self.variant.message();
            blue(format!("{s}--> "), &mut buffer);
            if let Some(path) = &self.path {
                blue(format!("{path}:"), &mut buffer);
            }
            blue(format!("{l}:{c}\n"), &mut buffer);
            blue(format!("{s} |\n"), &mut buffer);
            blue(format!("{l} | "), &mut buffer);
            white(format!("{line}\n"), &mut buffer);
            blue(format!("{s} | "), &mut buffer);
            red(format!("{underline} "), &mut buffer);
            red(format!("{message}\n"), &mut buffer);
            blue(format!("{s} |\n"), &mut buffer);
        };
        writer.print(&buffer).expect("uh oh");
        buffer.reset().expect("uh oh");
    }

    /// Returns the line that the error is on.
    fn line(&self) -> &str {
        self.line.as_str()
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
        let line_chars = self.line.chars();

        for c in line_chars.take(offset) {
            match c {
                '\t' => underline.push('\t'),
                _ => underline.push(' '),
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

// enum ErrorType {
//
// }