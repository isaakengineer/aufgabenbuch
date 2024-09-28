use sqlx::{
    query,
    sqlite::{SqliteConnection, SqlitePoolOptions, SqliteQueryResult},
    FromRow, Pool, Sqlite,
};
use std::sync::Mutex;
use tauri::{AppHandle, Builder, Emitter, Listener, Manager};
use tauri_plugin_dialog::{DialogExt, MessageDialogKind};

use chrono::{DateTime, NaiveDate};
use serde::{Deserialize, Serialize};

use crate::liste::AppData;

#[derive(Serialize, Deserialize, Clone, Debug, FromRow)]
pub struct Aufgabe {
    pub id: Option<i32>,        // integer (disabled)
    pub gruppe: Option<String>, // string (disabled)
    pub beschreibung: String,   // string (textarea)
    pub notiz: Option<String>,  // text (textarea)
    pub link: Option<String>,   // text (input)
    pub wochentag: u8,          // integer (drop down 0-7)
    pub prioritaet: u8,         // integer (drop down 0-4)
    pub position: Option<u32>,  // integer (disabled, wrapped in Option for nullable)

    pub verschoben: Option<DateTime<chrono::Utc>>, // date (checkbox)
    pub getan: Option<DateTime<chrono::Utc>>,      // date (checkbox)
    pub vernachlaessigt: Option<DateTime<chrono::Utc>>, // date (checkbox)
    pub kommentar: String,                         // string (input)

    pub erstellt_an: Option<DateTime<chrono::Utc>>, // date (disabled)
    pub geaendert_an: Option<DateTime<chrono::Utc>>, // date (disabled)
}

fn process_beschreibung(beschreibung: &str) -> Option<String> {
    if let Some(pos) = beschreibung.find('.') {
        let substring = &beschreibung[..pos];
        if substring.len() < 9 {
            return Some(substring.to_string());
        }
    }
    None
}

#[tauri::command]
pub async fn aufgabe_erledigen(app: AppHandle, aufgabe: Aufgabe) -> Result<(), String> {
    println!("Erledige Aufgabe: {:?}", aufgabe);

    let data = app.state::<Mutex<AppData>>();
    let db = data.lock().unwrap().pool.clone().unwrap();

    let query = include_str!("../queries/aufgabe_erledigen.sql");
    sqlx::query(query)
        .bind(aufgabe.verschoben)
        .bind(aufgabe.getan)
        .bind(aufgabe.vernachlaessigt)
        .bind(aufgabe.kommentar)
        // .bind(chrono::Utc::now().to_rfc3339()) // geändert an
        .bind(aufgabe.id)
        .execute(&db)
        .await
        .map_err(|e| format!("could not update Aufgabe {}", e))?;

    Ok(())
}

pub async fn letzte_aktualisierung( // falsche Name, sollte letzte neue Aufgabe sein!
    db: Pool<Sqlite>,
    res: SqliteQueryResult,
) -> Result<Aufgabe, String> {
    let id = res.last_insert_rowid();
    let aufgabe = sqlx::query_as::<_, Aufgabe>("SELECT * FROM liste WHERE id = ?1")
        .bind(id)
        .fetch_one(&db)
        .await
        .map_err(|e| format!("Failed to get todo {}", e))?;
    Ok(aufgabe)
}

#[tauri::command]
pub async fn aufgabe_aendern(app: AppHandle, aufgabe: Aufgabe) -> Result<Aufgabe, String> {
    let data = app.state::<Mutex<AppData>>();
    let db = data.lock().unwrap().pool.clone().unwrap();

    let gruppe = process_beschreibung(&aufgabe.beschreibung);

    let query = include_str!("../queries/aufgabe_aendern.sql");
    let result = sqlx::query(query)
        .bind(gruppe)
        .bind(aufgabe.beschreibung)
        .bind(aufgabe.notiz)
        .bind(aufgabe.link)
        .bind(aufgabe.wochentag)
        .bind(aufgabe.prioritaet)
        // .bind(chrono::Utc::now().to_rfc3339()) // geändert an
        .bind(aufgabe.id)
        .execute(&db)
        .await;

    match result {
        Ok(res) => {
        	let aufgabe_geaendert = sqlx::query_as::<_, Aufgabe>("SELECT * FROM liste WHERE id = ?1")
            .bind(aufgabe.id)
            .fetch_one(&db)
            .await
            .map_err(|e| format!("Könnte die Aufgabe mit ID {:?} nicht finden, weil {}", aufgabe.id, e))?;
            return Ok(aufgabe_geaendert);
        }
        Err(e) => {
            return Err(format!("Error saving todo: {}", e));
        }
    }
}

#[tauri::command]
pub async fn aufgabe_hinfuegen(app: AppHandle, aufgabe: Aufgabe) -> Result<Aufgabe, String> {
    println!("aufgabe_hinfuegen: {}", &aufgabe.beschreibung.clone());

    let data = app.state::<Mutex<AppData>>();
    let db = data.lock().unwrap().pool.clone().unwrap();

    let gruppe = process_beschreibung(&aufgabe.beschreibung);
    let result;
    if let Some(gruppe_value) = gruppe {
        let query = include_str!("../queries/aufgabe_hinfuegen_mit_gruppe.sql");
        result = sqlx::query(query)
            .bind(&gruppe_value)
            .bind(aufgabe.beschreibung)
            .bind(aufgabe.wochentag)
            .bind(aufgabe.prioritaet)
            // .bind(chrono::Utc::now().to_rfc3339()) // erstellt an
            .execute(&db)
            .await;
    } else {
        let query = include_str!("../queries/aufgabe_hinfuegen.sql");
        result = sqlx::query(query)
            .bind(aufgabe.beschreibung)
            .bind(aufgabe.wochentag)
            .bind(aufgabe.prioritaet)
            .execute(&db)
            .await;
    }
    match result {
        Ok(res) => {
            let aufgabe = letzte_aktualisierung(db, res).await?;
            return Ok(aufgabe);
        }
        Err(e) => {
            return Err(format!("Error saving todo: {}", e));
        }
    }
}

fn debug_liste(liste: Vec<Aufgabe>) {
    for aufgabe in liste {
        println!("{:?}\n-----------------------------", aufgabe);
    }
}
#[tauri::command]
pub async fn list_alle(app: AppHandle) -> Result<Vec<Aufgabe>, String> {
    let data = app.state::<Mutex<AppData>>();
    let db = data.lock().unwrap().pool.clone().unwrap();

    let query = include_str!("../queries/list_alle.sql");
    let liste: Vec<Aufgabe> = sqlx::query_as::<_, Aufgabe>(query)
        .fetch_all(&db)
        .await
        .map(|rows| rows.into_iter().collect())
        .map_err(|e| format!("Failed to get todos {}", e))?;
    if cfg!(dev) {
        debug_liste(liste.clone());
    }
    Ok(liste)
}

#[tauri::command]
pub async fn list_jetzige(app: AppHandle) -> Result<Vec<Aufgabe>, String> {
    let data = app.state::<Mutex<AppData>>();
    let db = data.lock().unwrap().pool.clone().unwrap();

    let query = include_str!("../queries/list_jetzige.sql");
    let liste: Vec<Aufgabe> = sqlx::query_as::<_, Aufgabe>(query)
        .fetch_all(&db)
        .await
        .map(|rows| rows.into_iter().collect())
        .map_err(|e| format!("Failed to get todos {}", e))?;

    if cfg!(dev) {
        debug_liste(liste.clone());
    }
    Ok(liste)
}

#[tauri::command]
pub async fn list_erledigt(app: AppHandle) -> Result<Vec<Aufgabe>, String> {
    let data = app.state::<Mutex<AppData>>();
    let db = data.lock().unwrap().pool.clone().unwrap();

    let liste: Vec<Aufgabe> = sqlx::query_as::<_, Aufgabe>("SELECT * FROM liste WHERE vernachlaessigt IS NOT NULL OR getan IS NOT NULL OR verschoben IS NOT NULL")
		.fetch_all(&db)
		.await
		.map(|rows| rows.into_iter().collect())
		.map_err(|e| format!("Failed to get todos {}", e))?;
    if cfg!(dev) {
        debug_liste(liste.clone());
    }
    Ok(liste)
}

#[tauri::command]
pub async fn gruppen_alle(app: AppHandle) -> Result<Vec<String>, String> {
    let data = app.state::<Mutex<AppData>>();
    let db = data.lock().unwrap().pool.clone().unwrap();

    let query = include_str!("../queries/gruppen_list.sql");
    let liste: Vec<String> = sqlx::query_scalar::<_, String>(query)
        .fetch_all(&db)
        .await
        .map(|rows| rows.into_iter().collect())
        .map_err(|e| format!("Failed to get todos {}", e))?;

    if cfg!(dev) {
        println!("{:?}", liste.clone());
    }
    Ok(liste)
}
