#[derive(Serialize, Deserialize)]
pub enum DbType {
    SQLITE
}

pub enum SqliteTypes {
    INT,
    INTEGER,
    TINYINT,
    SMALLINT,
    MEDIUMINT,
    BIGINT,
    INT2,
    INT8,
    CHARACTER,
    VARCHAR,
    NCHAR,
    NVARCHAR,
    TEXT,
    CLOB,
    BLOB,
    REAL,
    DOUBLE,
    FLOAT,
    NUMERIC,
    DECIMAL,
    BOOLEAN,
    DATE,
    DATETIME,
}