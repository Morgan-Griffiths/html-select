use scraper::{ElementRef, Html, Selector};
use std::error::Error;
use std::fmt;
use std::io::{BufRead, Result, Write};

pub struct CommandLineArgs {
    pub css_selector: String,
    pub input_file: Option<String>,
    pub output_file: Option<String>,
}

type CustomResultString<T> = std::result::Result<T, String>;

pub fn parse_command_line_arguments(args: Vec<String>) -> CustomResultString<CommandLineArgs> {
    let mut css_selector = None;
    let mut input_file = None;
    let mut output_file = None;

    let mut iter = args.into_iter().skip(1); // Skip the first argument (program name)

    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "-s" => {
                if let Some(selector) = iter.next() {
                    css_selector = Some(selector);
                } else {
                    return Err("Expected CSS selector after -s".to_string());
                }
            }
            "-i" => {
                if let Some(input) = iter.next() {
                    input_file = Some(input);
                } else {
                    return Err("Expected input file name after -i".to_string());
                }
            }
            "-o" => {
                if let Some(output) = iter.next() {
                    output_file = Some(output);
                } else {
                    return Err("Expected output file name after -o".to_string());
                }
            }
            _ => return Err(format!("Unexpected argument: {}", arg)),
        }
    }

    if let Some(selector) = css_selector {
        Ok(CommandLineArgs {
            css_selector: selector,
            input_file,
            output_file,
        })
    } else {
        Err("CSS selector is required. Use -s option followed by a selector.".to_string())
    }
}

pub fn read_input_html<R: BufRead>(mut input: R) -> Result<String> {
    let mut input_html = String::new();
    input.read_to_string(&mut input_html)?;
    Ok(input_html)
}

#[derive(Debug)]
pub struct CustomError(String);

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for CustomError {}

type CustomResult<T> = std::result::Result<T, CustomError>;

pub fn parse_html(html: &str) -> CustomResult<Html> {
    // You can add your custom error checking here, if needed.
    // For example, check if the HTML content is empty:
    if html.trim().is_empty() {
        Err(CustomError(
            "Failed to parse the provided HTML content.".to_string(),
        ))
    } else {
        Ok(Html::parse_document(html))
    }
}

pub fn apply_css_selector<'a>(
    html: &'a Html,
    selector: &'a str,
) -> CustomResultString<Vec<ElementRef<'a>>> {
    Selector::parse(selector)
        .map(|selector| html.select(&selector).collect())
        .map_err(|_| format!("Failed to parse the provided CSS selector: {}", selector))
}

pub fn write_output<W: Write>(mut output: W, elements: &[ElementRef]) -> Result<()> {
    for element in elements {
        writeln!(output, "{}", element.html())?;
    }
    Ok(())
}
