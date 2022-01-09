use super::schema::{get_create_table_query, get_insert_data_query, get_table_schema};

use std::fs;
use std::path::Path;

pub fn execute(arguments: Vec<String>) {
    let filename = &arguments[0];
    let tablename = filename.replace(".csv", "");
    let file_path = Path::new(&filename)
        .canonicalize()
        .expect("Couldn't get path for source file");

    let contents = fs::read_to_string(file_path).expect("Couldn't read from source file. :(");

    let table_schema = get_table_schema(&contents);
    let create_table = get_create_table_query(&tablename, &table_schema, &contents);
    let insert_data = get_insert_data_query(&tablename, &table_schema, &contents);

    fs::write("data.sql", format!("{}\n\n{}", create_table, insert_data))
        .expect("Couldn't write to file! :(");
}
