# Examples

## Running the app with sample data

To test the application with sample data, you can:

1. Execute the app in development mode passing the source file as an argument

```sh
$ cargo run pokemon.csv
```

2. Run the executable built using cargo with the file as an argument

```sh
$ cargo build --release
```

```sh
$ ./target/release/csv2sql $CSV_FILE_PATH
```
