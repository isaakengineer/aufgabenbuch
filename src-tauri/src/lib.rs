use tauri::{AppHandle, Emitter, Listener, Builder, Manager};
use tauri_plugin_dialog::{DialogExt, MessageDialogKind};
use std::sync::Mutex;
use sqlx::{sqlite::{SqliteConnection, SqlitePoolOptions}, Pool, Sqlite, FromRow};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

#[derive(Default)]
struct AppData {
    db_path: String,
    pool: Option<Pool<Sqlite>>,
}

use chrono::{NaiveDate, DateTime};
use serde::{Serialize, Deserialize};

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

use std::path::PathBuf;

 
fn file_waehlen_0(app: AppHandle) {
    println!("File waehlen");
    app.dialog().file().pick_file(move |file_path| {
        if let Some(file_path) = file_path {
            println!("Selected file: {}", file_path.to_string());

            let data = app.state::<Mutex<AppData>>();
            let mut data = data.lock().unwrap();

            data.db_path =  file_path.to_string();

            app.emit("file-gewaehlt", true).unwrap();
        } else {
            println!("No file selected");
        }
    });
}

#[tauri::command]
async fn file_waehlen(app: AppHandle) -> String {
    let app_handle = app.clone();
    app_handle.dialog().file().pick_file(move |file_path| {
        if let Some(file_path) = file_path {

            println!("Selected file: {}", file_path.to_string());
            
            let data = app_handle.state::<Mutex<AppData>>();
            let mut data = data.lock().unwrap();
            data.db_path =  file_path.to_string();

            app.emit("file-gewaehlt", true).unwrap();

        } else {
            
            println!("No file selected");
        
        }
    });

    // let data = app.state::<Mutex<AppData>>();
    // let data_lock = data.lock().unwrap();
    // let db_path = data_lock.db_path.clone();
    
    // println!("DB Path: {}", db_path);
    // format!("DB Path: {}", db_path)
    format!("lass uns mal sehen")
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
async fn aufgabe_hinfuegen(app: AppHandle, beschreibung: &str) -> Result<(), String> {

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

#[tauri::command]
async fn list_alle(app: AppHandle) -> Result<Vec<Aufgabe>, String> {
    
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

fn debug_liste(liste: Vec<Aufgabe>) {
    for aufgabe in liste {
        println!("{:?}\n-----------------------------", aufgabe);
    }
}

#[tauri::command]
async fn list(app: AppHandle) -> String {
    println!("list");

    // 
    let data = app.state::<Mutex<AppData>>();
    let db_path = data.lock().unwrap().db_path.clone();
    // println!("DB Path: {}", &db_path);
    // drop(data_lock);

    let pool = SqlitePoolOptions::new()
        .connect(&db_path).await.unwrap();

    sqlx::migrate!("./migrations").run(&pool).await.unwrap();

    let data = app.state::<Mutex<AppData>>();
    let mut data = data.lock().unwrap();
    data.pool = Some(pool);
    

    format!("DB Path:")
}

#[tauri::command]
async fn greet(app: AppHandle, name: String) -> String {
    let data = app.state::<Mutex<AppData>>();
    let data_lock = data.lock().unwrap();
    let db_path = data_lock.db_path.clone();
    format!("DB Path: {}", db_path)
}

// async fn database_init(app: AppHandle) {

//     let data = app.state::<Mutex<AppData>>();
    
//     
//     let mut data = data.lock().unwrap();

//     data.pool = Some(pool.unwrap());

//     let app_data = app.state::<Mutex<AppData>>();
//     let db = app_data.lock().unwrap().db_path.clone();
//     sqlx::migrate!("./migrations").run(&db).await.unwrap();
// }


#[tauri::command] // JUST SOME INITIAL TESTS with dialog.message AND ipc
fn greet_2(app: AppHandle, name: String) -> String {
    let message = format!("Hello, {}! You've been greeted from Rust!", name);
//     // let file_path = app.dialog().file().blocking_pick_file();

//     // format!("Hello, {}! You've been greeted from Rust!", name);
//     // format!("{} has selected the file", name)

//     // app.dialog()
//     //     .message("Tauri is Awesome")
//     //     .kind(MessageDialogKind::Info)
//     //     .title("Information")
//     //     .ok_button_label("Absolutely")
//     //     .show(|result| match result {
//     //         true => (), // do something,
//     //         false => () // do something,
//     //     });
//     // let message = app.dialog().file().pick_file(move |file_path| {
//     //     match file_path {
//     //         None => "No file selected".to_string(),
//     //         Some(file_path) => "hi there".to_string(),
//     //     }
//     // });
//     let message = {
//         let rt = Runtime::new().unwrap();
//         let result = rt.block_on(async {
//             app.dialog().file().pick_file(move |file_path| {
//                 match file_path {
//                     None => "No file selected".to_string(),
//                     Some(file_path) => "hi there".to_string(),
//                 }
//             }).await
//         });

//         result.unwrap_or("Error selecting file".to_string())
//     };

    message
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
#[tokio::main]
pub async fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            file_waehlen,
            list,
            aufgabe_hinfuegen,
            list_alle,
        ])
        .setup(|app| {
            // let handle = app.handle().clone();
            app.manage(Mutex::new(AppData::default()));
            let handle_clone = app.handle().clone();
            app.listen("file-waehlen", move |event| {
                if let Ok(flag) = serde_json::from_str::<bool>(event.payload()) {
                    file_waehlen_0(handle_clone.clone(), );
                    // print_db_path(handle_clone.clone());
                    println!("Flag: {}", flag.to_string());
                    
                    handle_clone.unlisten(event.id());
                }
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
