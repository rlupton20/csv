fn main() {
    println!("Hello, world!");
}

fn get_columns(s: &String, sep: char) -> Vec<&str> {
    s.trim().split(sep).map(|s| s.trim()).collect()
}

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


struct SortedVector {
    v: Vec<usize>,
}

impl SortedVector {
    fn new() -> SortedVector {
        SortedVector { v: vec![] }
    }

    fn insert(mut self, x: usize) -> SortedVector {
        match self.v.binary_search(&x) {
            Ok(p) => self.v.insert(p, x),
            Err(p) => self.v.insert(p, x),
        }
        self
    }
}

#[test]
fn test_sorted_vector_new_insert() {
    let result: Vec<usize> = SortedVector::new().insert(1).insert(2).v;
    let expected = vec![1, 2];
    assert_eq!(expected, result);
}

#[test]
fn test_sorted_vector_existing_insert() {
    let result: Vec<usize> = SortedVector::new()
        .insert(0)
        .insert(1)
        .insert(1)
        .insert(2)
        .v;
    let expected = vec![0, 1, 1, 2];
    assert_eq!(expected, result);
}


fn get_column_indices(headings: &Vec<&str>, seek: &Vec<&str>) -> Vec<usize> {
    let mut indices: SortedVector = SortedVector::new();
    for c in seek.iter() {
        let i: usize = headings.binary_search(c).expect("Heading not present");
        indices = indices.insert(i);
    }
    indices.v
}

#[test]
fn test_get_column_indices() {
    let headings: Vec<&str> = vec!["foo", "bar", "baz", "bif"];
    let seek: Vec<&str> = vec!["bar", "bif"];
    let result: Vec<usize> = get_column_indices(&headings, &seek);
    let expected: Vec<usize> = vec![1, 3];
    assert_eq!(expected, result);
}
