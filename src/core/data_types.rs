pub enum SQLType {
    TEXT,
    INTEGER,
    DOUBLE,
    BOOLEAN,
}

impl std::fmt::Debug for SQLType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl std::string::ToString for SQLType {
    fn to_string(&self) -> String {
        let val = match self {
            &SQLType::BOOLEAN => "BOOLEAN",
            &SQLType::DOUBLE => "DOUBLE",
            &SQLType::INTEGER => "INTEGER",
            &SQLType::TEXT => "TEXT",
        };

        String::from(val)
    }
}
