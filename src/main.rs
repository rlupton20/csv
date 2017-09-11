extern crate clap;

use clap::{App, Arg};
use std::io;

use std::iter::Peekable;

fn main() {
    let matches = App::new("csv")
        .version("0.1")
        .about("Extracts columns from CSV files")
        .author("Richard Lupton 2017")
        .arg(
            Arg::with_name("columns")
                .multiple(true)
                .takes_value(true)
                .required(true)
                .number_of_values(1),
        )
        .get_matches();

    let seek: Vec<&str> = matches.values_of("columns").unwrap().collect();

    let mut buffer: String = String::new();
    let mut header_buffer: String = String::new();

    let stdin: io::Stdin = io::stdin();
    stdin.read_line(&mut header_buffer).expect(
        "Failed to read stdin",
    );

    let headings: Vec<&str> = get_columns(&header_buffer, ',');
    let indices: Vec<usize> = get_column_indices(&headings, &seek);

    // Print column headings
    for (v, last) in FlagLast::from_iter(seek.iter()) {
        print!("{}{}", v, if last { "\n" } else { "," });
    }

    // Print filtered columns
    loop {
        buffer.clear();
        if stdin.read_line(&mut buffer).expect("Failed to read stdin") == 0 {
            break;
        }

        let columns: Vec<&str> = get_columns(&buffer, ',');
        for (i, last) in FlagLast::from_iter(indices.iter()) {
            print!("{}{}", columns[*i], if last { "\n" } else { "," });
        }
    }
}


fn get_columns(s: &str, sep: char) -> Vec<&str> {
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

struct FlagLast<I>
where
    I: Iterator,
{
    iterator: Peekable<I>,
}

impl<I> FlagLast<I>
where
    I: Iterator,
{
    fn from_iter(i: I) -> FlagLast<I> {
        FlagLast { iterator: i.peekable() }
    }
}

impl<I> Iterator for FlagLast<I>
where
    I: Iterator,
{
    type Item = (<I as Iterator>::Item, bool);

    fn next(&mut self) -> Option<(<I as Iterator>::Item, bool)> {
        match self.iterator.next() {
            None => None,
            Some(v) => {
                match self.iterator.peek() {
                    None => Some((v, true)),
                    Some(_) => Some((v, false)),
                }
            }
        }
    }
}



// *****************************************************************************
// * Tests
// *****************************************************************************
#[test]
fn test_get_columns_with_commas() {
    let line: &str = "foo, bar, baz";
    let result: Vec<&str> = get_columns(&line, ',');
    let expected: Vec<&str> = vec!["foo", "bar", "baz"];
    assert_eq!(expected, result);
}

#[test]
fn test_get_columns_with_semicolons() {
    let line: &str = "foo;bar;baz";
    let result: Vec<&str> = get_columns(&line, ';');
    let expected: Vec<&str> = vec!["foo", "bar", "baz"];
    assert_eq!(expected, result);
}

#[test]
fn test_get_columns_strips_header_whitespace() {
    let line: &str = "   foo,  bar,    baz";
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

#[test]
fn test_flag_last_iterator_by_step_through() {
    let v = vec![1, 2, 3];
    let mut iterator = FlagLast::from_iter(v.iter());

    if let Some((n, b)) = iterator.next() {
        assert_eq!(1, *n);
        assert_eq!(false, b);
    } else {
        assert!(false);
    }

    if let Some((n, b)) = iterator.next() {
        assert_eq!(2, *n);
        assert_eq!(false, b);
    } else {
        assert!(false);
    }

    if let Some((n, b)) = iterator.next() {
        assert_eq!(3, *n);
        assert_eq!(true, b);
    } else {
        assert!(false);
    }

    if let None = iterator.next() {
        // All is well
    } else {
        assert!(false);
    }

}
