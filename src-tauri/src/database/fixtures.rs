use rusqlite::{Connection, Result};

pub fn create_tables() -> Result<()> {
    let connection = Connection::open("./.my-board-db.db")?;

    let create_states_table_query = r#"CREATE TABLE IF NOT EXISTS states(
            id INTEGER PRIMARY KEY
            , name VARCHAR(255) NOT NULL
            , color VARCHAR(50) DEFAULT NULL
            , position INTEGER NOT NULL
            , board_id UNSIGNED INTEGER NOT NULL
        )"#;
    connection.execute(create_states_table_query, ())?;

    let create_boards_table_query = r#"CREATE TABLE IF NOT EXISTS boards(
            id INTEGER PRIMARY KEY
            , name VARCHAR(255) NOT NULL
            , position INTEGER NOT NULL
            , group_id BIGINT NOT NULL
        )"#;
    connection.execute(&create_boards_table_query, ())?;

    let create_groups_table_query = r#"CREATE TABLE IF NOT EXISTS groups(
        id INTEGER PRIMARY KEY
        , name VARCHAR(255) NOT NULL
        , icon VARCHAR(255) DEFAULT NULL
        , position INTEGER NOT NULL
    )"#;
    connection.execute(&create_groups_table_query, ())?;
    let create_tasks_table_query = r#"CREATE TABLE IF NOT EXISTS board(
        id INTEGER PRIMARY KEY
        , name VARCHAR(255) NOT NULL
        , description LONGTEXT DEFAULT NULL
        , duration INTEGER NOT NULL
        , progress INTEGER DEFAULT NULL
        , priority TINY INT
        , state_id INTEGER NOT NULL
        , board_id BIGINT NOT NULL
        , position INTEGER NOT NULL
        , started_at TEXT DEFAULT NULL
        , ended_at TEXT DEFAULT NULL
    )"#;
    connection.execute(&create_tasks_table_query, ())?;

    println!("created all tables!");
    return Ok(());
}
