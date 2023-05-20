use std::env;
use std::fs::File;
use std::io::{self, BufReader, BufWriter};

use html_select::{
    apply_css_selector, parse_command_line_arguments, parse_html, read_input_html, write_output,
};

fn main() {
    let args: Vec<String> = env::args().collect();

    match parse_command_line_arguments(args) {
        Ok(cli_args) => {
            // Read input HTML
            let input: Box<dyn io::BufRead> = match cli_args.input_file {
                Some(file_name) => Box::new(BufReader::new(File::open(file_name).unwrap())),
                None => Box::new(io::stdin().lock()),
            };
            let input_html = read_input_html(input).unwrap();

            // Parse HTML
            let parsed_html = parse_html(&input_html).unwrap();

            // Apply CSS selector
            let elements = apply_css_selector(&parsed_html, &cli_args.css_selector).unwrap();

            // Write output
            let output: Box<dyn io::Write> = match cli_args.output_file {
                Some(file_name) => Box::new(BufWriter::new(File::create(file_name).unwrap())),
                None => Box::new(io::stdout()),
            };
            write_output(output, &elements).unwrap();
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
