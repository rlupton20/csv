# csv
A tool for easy processing of CSV files. Currently filters stdin to stdout.

## Usage

Given a csv file

```
$ cat test.csv
foo,bar,baz
1,2,3
4,5,6
7,8,9
```

you can filter it through csv by providing a list of column names

```
$ cat test.csv | csv foo baz
foo,baz
1,3
4,6
7,9
```

## Building from source

You will need the rust toolchain (in particular, cargo) installed. Build with

```
cargo build --release
```

The built binary can then be found in `target/release` and added to your path in a suitable place (e.g. `/usr/local/bin`).
