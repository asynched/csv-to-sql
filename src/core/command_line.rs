use super::schema::{get_create_table_query, get_insert_data_query, get_table_schema};

use std::fs;

pub fn execute() {
    let contents = fs::read_to_string("data.csv").expect("Couldn't read from source file. :(");

    let table_schema = get_table_schema(&contents);
    let create_table = get_create_table_query("data", &table_schema, &contents);
    let insert_data = get_insert_data_query("data", &table_schema, &contents);

    fs::write("data.sql", format!("{}\n\n{}", create_table, insert_data))
        .expect("Couldn't write to file! :(");
}
