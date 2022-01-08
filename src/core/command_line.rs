use super::schema::{get_create_table_query, get_insert_data_query};

use std::fs;

pub fn execute() -> Result<(), std::io::Error> {
    // Parse to SQL
    // 1. Get table schema - DONE
    // 2. Generate a query to create the table schema DONE
    // 3. Generate the query to insert the data
    let contents = fs::read_to_string("data.csv")?;
    let create_table = get_create_table_query("data", &contents);
    let insert_data = get_insert_data_query("data", &contents);

    fs::write("data.sql", format!("{}\n\n{}", create_table, insert_data))?;

    Ok(())
}
