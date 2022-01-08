use regex::Regex;

use super::data_types::SQLType;

pub fn get_insert_data_query(tablename: &str, file_contents: &String) -> String {
    let insert_query = format!("INSERT INTO\n\t{}\nVALUES\n\t", tablename);

    let data = file_contents
        .split('\n')
        .skip(1)
        .map(|item| format!("(NULL, {})", item))
        .collect::<Vec<String>>()
        .join(",\n\t");

    format!("{}{}", insert_query, data)
}

pub fn get_create_table_query(tablename: &str, file_contents: &String) -> String {
    let lines = file_contents
        .split('\n')
        .map(String::from)
        .collect::<Vec<String>>();

    let row = &lines[1];
    let data_types = analyze_row(row);
    let columns = parse_columns(&lines[0]);

    let create_table_query = generate_create_table(tablename, &columns, &data_types);

    create_table_query
}

fn generate_create_table(
    tablename: &str,
    columns: &Vec<String>,
    data_types: &Vec<SQLType>,
) -> String {
    let create_table_query = format!(
        "CREATE TABLE {} (\n\tid INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,\n\t",
        tablename
    );

    let mut rows: Vec<String> = Vec::new();

    for (data_type, column) in data_types.iter().zip(columns) {
        let column = format!("`{}` {}", column, data_type.to_string());
        rows.push(column);
    }

    let parsed_rows = rows.join(",\n\t");
    format!("{}{}\n);", create_table_query, parsed_rows)
}

fn parse_columns(columns: &String) -> Vec<String> {
    columns
        .split(',')
        .map(String::from)
        .collect::<Vec<String>>()
}

fn analyze_row(row: &String) -> Vec<SQLType> {
    row.split(',')
        .map(get_column_type)
        .collect::<Vec<SQLType>>()
}

fn get_column_type(column: &str) -> SQLType {
    let integer_match = Regex::new(r"^[\d]+$").unwrap();
    let double_match = Regex::new(r"^-?(\d+\.?\d+)$").unwrap();

    if integer_match.is_match(column) {
        return SQLType::INTEGER;
    }

    if double_match.is_match(column) {
        return SQLType::DOUBLE;
    }

    match column {
        "true" | "false" => SQLType::BOOLEAN,
        _ => SQLType::TEXT,
    }
}
