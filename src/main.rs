use std::io;

fn main() {
    let mut buffer: String = String::new();
    let mut header_buffer: String = String::new();
    let stdin: io::Stdin = io::stdin();
    stdin.read_line(&mut header_buffer).expect(
        "Failed to read stdin",
    );

    let seek: Vec<&str> = vec!["foo", "baz"];
    let headings: Vec<&str> = get_columns(&header_buffer, ',');
    let indices: Vec<usize> = get_column_indices(&headings, &seek);

    loop {
        buffer.clear();
        if stdin.read_line(&mut buffer).expect("Failed to read stdin") == 0 {
            break;
        }
        let columns: Vec<&str> = get_columns(&buffer, ',');

        let mut iterator = indices.iter().peekable();
        while let Some(v) = iterator.next() {
            print!("{}", columns[*v]);
            match iterator.peek() {
                None => print!("\n"),
                Some(_) => print!(", "),
            }
        }
    }
}


fn get_columns(s: &String, sep: char) -> Vec<&str> {
    s.trim().split(sep).map(|s| s.trim()).collect()
}

fn locate(s: &str, v: &Vec<&str>) -> Option<usize> {
    for (i, x) in v.iter().enumerate() {
        if *x == s {
            return Some(i);
        }
    }
    None
}

fn get_column_indices(headings: &Vec<&str>, seek: &Vec<&str>) -> Vec<usize> {
    let mut indices: Vec<usize> = vec![];
    for c in seek.iter() {
        let i: usize = locate(c, headings).expect("Heading not present");
        indices.push(i);
    }
    indices
}



// *****************************************************************************
// * Tests
// *****************************************************************************
#[test]
fn test_get_columns_with_commas() {
    let line: String = String::from("foo,bar,baz");
    let result: Vec<&str> = get_columns(&line, ',');
    let expected: Vec<&str> = vec!["foo", "bar", "baz"];
    assert_eq!(expected, result);
}

#[test]
fn test_get_columns_with_semicolons() {
    let line: String = String::from("foo;bar;baz");
    let result: Vec<&str> = get_columns(&line, ';');
    let expected: Vec<&str> = vec!["foo", "bar", "baz"];
    assert_eq!(expected, result);
}

#[test]
fn test_get_columns_strips_header_whitespace() {
    let line: String = String::from("   foo,  bar,    baz");
    let result: Vec<&str> = get_columns(&line, ',');
    let expected: Vec<&str> = vec!["foo", "bar", "baz"];
    assert_eq!(expected, result);
}

#[test]
fn test_locate_finds_string_at_correct_index() {
    let v: Vec<&str> = vec!["foo", "bar", "baz"];
    let result: Option<usize> = locate("bar", &v);
    let expected: Option<usize> = Some(1);
    assert_eq!(expected, result);
}

#[test]
fn test_locate_returns_failure_on_non_existant_string() {
    let v: Vec<&str> = vec!["foo", "bar", "baz"];
    let result: Option<usize> = locate("foobar", &v);
    let expected: Option<usize> = None;
    assert_eq!(expected, result);
}

#[test]
fn test_get_column_indices() {
    let headings: Vec<&str> = vec!["foo", "bar", "baz", "bif"];
    let seek: Vec<&str> = vec!["bar", "bif"];
    let result: Vec<usize> = get_column_indices(&headings, &seek);
    let expected: Vec<usize> = vec![1, 3];
    assert_eq!(expected, result);
}
