// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{generate_context, Builder};
use tauri_plugin_sql::{Builder as SqlBuilder, Migration, MigrationKind};

fn main() {
    let migrations = vec![Migration {
        version: 1,
        description: "create_comments_table",
        sql: r#"
                CREATE TABLE IF NOT EXISTS comments (
                    id SERIAL PRIMARY KEY,
                    text TEXT NOT NULL,
                    created_at TIMESTAMPTZ DEFAULT now()
                )
            "#,
        kind: MigrationKind::Up,
    }];

    tauri::Builder::default()
        .plugin(
            SqlBuilder::default()
                .add_migrations(
                    "postgres://user:password@localhost:5432/pomodoro_fate",
                    migrations,
                )
                .build(),
        )
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
