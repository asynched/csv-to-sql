use regex::Regex;

use super::data_types::SQLType;

fn parse_row(row: &str, table_schema: &Vec<SQLType>) -> String {
    let mut parsed_contents: Vec<String> = Vec::new();
    let contents = row.split(',').map(String::from).collect::<Vec<String>>();

    for (item, data_type) in contents.iter().zip(table_schema) {
        let parsed_column = match data_type {
            &SQLType::TEXT => format!("'{}'", item),
            _ => item.to_owned(),
        };

        parsed_contents.push(parsed_column);
    }

    let parsed_row = parsed_contents.join(", ");

    format!("(NULL, {})", parsed_row)
}

pub fn get_insert_data_query(
    tablename: &str,
    table_schema: &Vec<SQLType>,
    file_contents: &String,
) -> String {
    let insert_query = format!("INSERT INTO\n\t{}\nVALUES\n\t", tablename);

    let data = file_contents
        .split('\n')
        .skip(1)
        .map(|item| item.trim().replace("'", "''"))
        .map(|item| parse_row(item.as_str(), table_schema))
        .collect::<Vec<String>>()
        .join(",\n\t");

    format!("{}{};\n", insert_query, data)
}

pub fn get_table_schema(file_contents: &String) -> Vec<SQLType> {
    let heading = file_contents
        .split('\n')
        .skip(1)
        .take(1)
        .collect::<String>();

    let data_types = analyze_row(&heading);

    data_types
}

pub fn get_create_table_query(
    tablename: &String,
    table_schema: &Vec<SQLType>,
    file_contents: &String,
) -> String {
    let lines = file_contents
        .split('\n')
        .map(String::from)
        .collect::<Vec<String>>();

    let columns = parse_columns(&lines[0]);
    let create_table_query = generate_create_table(tablename, &columns, &table_schema);
    create_table_query
}

fn generate_create_table(
    tablename: &String,
    columns: &Vec<String>,
    data_types: &Vec<SQLType>,
) -> String {
    let create_table_query = format!(
        "CREATE TABLE {} (\n\tid INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,\n\t",
        tablename
    );

    let mut rows: Vec<String> = Vec::new();

    for (data_type, column) in data_types.iter().zip(columns) {
        let column = format!("`{}` {}", column.trim(), data_type.to_string());
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
