use super::schema::{get_create_table_query, get_insert_data_query, get_table_schema};

use std::fs;
use std::path::Path;

pub fn execute(arguments: Vec<String>) {
    let filename = &arguments[0];
    let tablename = filename.replace(".csv", "");
    let source_file_path = Path::new(&filename)
        .canonicalize()
        .expect("Couldn't get path for source file");

    let output_file = format!("{}.sql", tablename);
    let output_file_path = Path::new(&output_file);

    let contents =
        fs::read_to_string(source_file_path).expect("Couldn't read from source file. :(");

    let table_schema = get_table_schema(&contents);
    let create_table = get_create_table_query(&tablename, &table_schema, &contents);
    let insert_data = get_insert_data_query(&tablename, &table_schema, &contents);

    fs::write(
        output_file_path,
        format!("{}\n\n{}", create_table, insert_data),
    )
    .expect("Couldn't write to file! :(");
}
