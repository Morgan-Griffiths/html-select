use crate::{parse_command_line_arguments, CommandLineArgs};

#[test]
fn test_parse_command_line_arguments() {
    let valid_args1 = vec![
        String::from("program_name"),
        String::from("-s"),
        String::from(".class_name"),
    ];
    let parsed_args1 = parse_command_line_arguments(valid_args1).unwrap();
    assert_eq!(parsed_args1.css_selector, ".class_name");
    assert_eq!(parsed_args1.input_file, None);
    assert_eq!(parsed_args1.output_file, None);

    let valid_args2 = vec![
        String::from("program_name"),
        String::from("-s"),
        String::from(".class_name"),
        String::from("-i"),
        String::from("input.html"),
    ];
    let parsed_args2 = parse_command_line_arguments(valid_args2).unwrap();
    assert_eq!(parsed_args2.css_selector, ".class_name");
    assert_eq!(parsed_args2.input_file, Some(String::from("input.html")));
    assert_eq!(parsed_args2.output_file, None);

    let valid_args3 = vec![
        String::from("program_name"),
        String::from("-s"),
        String::from(".class_name"),
        String::from("-o"),
        String::from("output.html"),
    ];
    let parsed_args3 = parse_command_line_arguments(valid_args3).unwrap();
    assert_eq!(parsed_args3.css_selector, ".class_name");
    assert_eq!(parsed_args3.input_file, None);
    assert_eq!(parsed_args3.output_file, Some(String::from("output.html")));

    let valid_args4 = vec![
        String::from("program_name"),
        String::from("-s"),
        String::from(".class_name"),
        String::from("-i"),
        String::from("input.html"),
        String::from("-o"),
        String::from("output.html"),
    ];
    let parsed_args4 = parse_command_line_arguments(valid_args4).unwrap();
    assert_eq!(parsed_args4.css_selector, ".class_name");
    assert_eq!(parsed_args4.input_file, Some(String::from("input.html")));
    assert_eq!(parsed_args4.output_file, Some(String::from("output.html")));

    let invalid_args1 = vec![String::from("program_name")];
    assert!(parse_command_line_arguments(invalid_args1).is_err());

    let invalid_args2 = vec![
        String::from("program_name"),
        String::from("-s"),
        String::from(".class_name"),
        String::from("-x"),
    ];
    assert!(parse_command_line_arguments(invalid_args2).is_err());
}

use crate::read_input_html;
use std::fs::File;
use std::io::Cursor;
use std::io::{self, Cursor, Write};
use std::path::Path;
use tempfile::NamedTempFile;

#[test]
fn test_read_input_html() {
    let input_data = "<html><head></head><body><p>Hello, world!</p></body></html>";
    let input_cursor = Cursor::new(input_data);

    let html_content = read_input_html(input_cursor).unwrap();
    assert_eq!(html_content, input_data);

    // Test reading from a file
    let mut tmp_file = NamedTempFile::new().expect("Failed to create temporary file");
    write!(tmp_file, "{}", input_data).expect("Failed to write to temporary file");
    let file_path = tmp_file.path().to_path_buf();
    tmp_file.sync_all().expect("Failed to sync temporary file");

    let file = File::open(&file_path).expect("Failed to open temporary file");
    let html_content_from_file = read_input_html(io::BufReader::new(file)).unwrap();
    assert_eq!(html_content_from_file, input_data);

    // Test error handling
    struct FailingReader;

    impl io::Read for FailingReader {
        fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
            Err(io::Error::new(io::ErrorKind::Other, "Simulated I/O error"))
        }
    }

    let failing_reader = FailingReader;
    let read_result = read_input_html(failing_reader);
    assert!(read_result.is_err());
}

use crate::parse_html;
use scraper::Selector;

#[test]
fn test_parse_html() {
    let input_data = r#"
        <html>
            <head></head>
            <body>
                <h1 id="title">Hello, world!</h1>
                <ul class="list">
                    <li>Item 1</li>
                    <li>Item 2</li>
                </ul>
            </body>
        </html>
    "#;

    let parsed_html = parse_html(input_data).unwrap();

    // Test parsing by querying elements using CSS selectors
    let title_selector = Selector::parse("#title").unwrap();
    let title_element = parsed_html.select(&title_selector).next().unwrap();
    assert_eq!(
        title_element.text().collect::<Vec<_>>().join(""),
        "Hello, world!"
    );

    let list_selector = Selector::parse(".list > li").unwrap();
    let list_items: Vec<String> = parsed_html
        .select(&list_selector)
        .map(|el| el.text().collect::<Vec<_>>().join(""))
        .collect();
    assert_eq!(list_items, vec!["Item 1", "Item 2"]);

    // Additional test cases can be added, for example:
    // - Testing with more complex HTML input
    // - Testing error handling (e.g., invalid HTML input)
}

use crate::{apply_css_selector, parse_html, write_output};
use std::io::Cursor;

#[test]
fn test_write_output() {
    let input_data = r#"
        <html>
            <head></head>
            <body>
                <h1 id="title">Hello, world!</h1>
                <ul class="list">
                    <li>Item 1</li>
                    <li>Item 2</li>
                </ul>
            </body>
        </html>
    "#;

    let parsed_html = parse_html(input_data).unwrap();
    let elements = apply_css_selector(&parsed_html, "#title").unwrap();

    // Test writing output to a Cursor
    let mut output_cursor = Cursor::new(Vec::new());
    write_output(&mut output_cursor, &elements).unwrap();

    let output_data = output_cursor.into_inner();
    let output_string = String::from_utf8(output_data).unwrap();
    assert_eq!(output_string, "<h1 id=\"title\">Hello, world!</h1>");

    // Additional test cases can be added, for example:
    // - Writing multiple elements to the output
    // - Writing to a file instead of a Cursor (requires a temporary file)
    // - Testing error handling (e.g., for I/O errors)
}
