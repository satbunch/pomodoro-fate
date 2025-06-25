// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use dotenvy::dotenv;
use std::env;
use tauri::{generate_context, Builder};
use tauri_plugin_sql::{Builder as SqlBuilder, Migration, MigrationKind};

fn main() {
    dotenv().ok();

    let database_url =
        env::var("VITE_DATABASE_URL").expect("`.env`にDATABASE_URLが設定されていません");

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

    Builder::default()
        .plugin(
            SqlBuilder::default()
                .add_migrations(&database_url, migrations)
                .build(),
        )
        .invoke_handler(tauri::generate_handler![])
        .run(generate_context!())
        .expect("error while running tauri application");
}
