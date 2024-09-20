use tauri::{AppHandle, Emitter, Listener, Builder, Manager};
use tauri_plugin_dialog::{DialogExt, MessageDialogKind};
use std::sync::Mutex;
use sqlx::{sqlite::{SqliteConnection, SqlitePoolOptions}, Pool, Sqlite, FromRow};

use chrono::{NaiveDate, DateTime};
use serde::{Serialize, Deserialize};

use crate::liste::AppData;

#[derive(Serialize, Deserialize, Clone, Debug, FromRow)]
pub struct Aufgabe {
    pub id: Option<i32>,                 // integer (disabled)
    pub gruppe: Option<String>,          // string (disabled)
    pub beschreibung: String,            // string (textarea)
    pub notiz: Option<String>,           // text (textarea)
    pub link: Option<String>,            // text (input)
    pub wochentag: u8,                   // integer (drop down 0-7)
    pub prioritaet: u8,                  // integer (drop down 0-4)
    pub position: Option<u32>,           // integer (disabled, wrapped in Option for nullable)

    pub verschoben: Option<DateTime<chrono::Utc>>, // date (checkbox)
    pub getan: Option<DateTime<chrono::Utc>>,      // date (checkbox)
    pub vernachlaessigt: Option<DateTime<chrono::Utc>>, // date (checkbox)
    pub kommentar: String,               // string (input)

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

    sqlx::query("UPDATE liste SET verschoben = ?1, getan = ?2, vernachlaessigt = ?3, kommentar = ?4 WHERE id = ?5")
        .bind(aufgabe.verschoben)
        .bind(aufgabe.getan)
        .bind(aufgabe.vernachlaessigt)
        .bind(aufgabe.kommentar)
        .bind(aufgabe.id)
        .execute(&db)
        .await
        .map_err(|e| format!("could not update Aufgabe {}", e))?;

    Ok(())
}

#[tauri::command]
pub async fn aufgabe_hinfuegen(app: AppHandle, beschreibung: &str) -> Result<(), String> {

    println!("aufgabe_hinfuegen: {}", &beschreibung);

    let data = app.state::<Mutex<AppData>>();
    let db = data.lock().unwrap().pool.clone().unwrap();

    let gruppe = process_beschreibung(&beschreibung);
    if let Some(gruppe_value) = gruppe {
        sqlx::query("INSERT INTO liste (gruppe, beschreibung, wochentag, prioritaet) VALUES (?1, ?2, 0, 0)")
            .bind(&gruppe_value)
            .bind(beschreibung)
            .execute(&db)
            .await
            .map_err(|e| format!("Error saving todo: {}", e))?;
    } else {
        sqlx::query("INSERT INTO liste (beschreibung, wochentag, prioritaet) VALUES (?1, 0, 0)")
            .bind(beschreibung)
            .execute(&db)
            .await
            .map_err(|e| format!("Error saving todo: {}", e))?;
    }
    Ok(())
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

    let liste: Vec<Aufgabe> = sqlx::query_as::<_, Aufgabe>("SELECT * FROM liste")
        .fetch_all(&db)
        .await
        .map(|rows| rows.into_iter().collect())
        .map_err(|e| format!("Failed to get todos {}", e))?;
    if cfg!(dev) {
        debug_liste(liste.clone());
    }
    Ok(liste)
}