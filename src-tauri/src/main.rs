
use std::fs;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use std::io::Error;
use rusqlite::{params, Connection, Result, Error as RusqliteError};



#[derive(Debug, Serialize, Deserialize, Clone)]
struct Note {
  id: i32,
  title: String,
  content: String,
  date: String,
}

// Fonction pour créer une nouvelle note et la stocker dans un fichier JSON
#[tauri::command]
fn create_note(title: String, content: String) -> Result<Vec<Note>, String> {
  // Créer la nouvelle note
  let id = generate_unique_id();
  let date = Utc::now();
  let date_str = date.format("%d/%m/%Y").to_string();

  let new_note = Note {
    id,
    title,
    content,
    date: date_str,
  };

  // Lire les notes existantes depuis le fichier JSON
  let mut notes = read_notes().unwrap_or_else(|err| {
    eprintln!("Erreur lors de la lecture des notes: {}", err);
    Vec::new() // Si une erreur se produit lors de la lecture des notes, créer une nouvelle liste vide
  });

  // Ajouter la nouvelle note à la liste des notes
  notes.push(new_note.clone());

  // Écrire la liste mise à jour dans le fichier JSON
  if let Err(err) = write_notes(&notes) {
    eprintln!("Erreur lors de l'écriture des notes: {}", err);
    return Err("Erreur lors de l'écriture des notes".to_string());
  }

  // Lire le fichier à nouveau et renvoyer les données
  match read_notes() {
    Ok(updated_notes) => {
      println!("Notes mises à jour : {:?}", updated_notes);
      Ok(updated_notes)
    }
    Err(err) => {
      eprintln!("Erreur lors de la lecture des notes mises à jour: {}", err);
      Err("Erreur lors de la lecture des notes mises à jour".to_string())
    }
  }
}

fn generate_unique_id() -> i32 {
  // Lire les notes depuis le fichier JSON
  let notes_result = read_notes();

  // Vérifier s'il y a une erreur lors de la lecture des notes
  if let Err(err) = notes_result {
    eprintln!("Erreur lors de la récupération des notes: {}", err);
    return -1; // Retourner une valeur d'erreur
  }

  let notes: Vec<Note> = notes_result.unwrap_or_else(|_| vec![]);


  // Récupérer l'ID de la dernière note
  let last_note_id = notes.last().map_or(0, |note| note.id);

  // Générer un nouvel ID unique en incrémentant l'ID de la dernière note
  let new_id = last_note_id + 1;

  new_id
}

#[tauri::command]
fn fetch_notes() -> Result<Vec<Note>, String> {
  // Lire les notes depuis le fichier JSON
  let notes = read_notes()?;
  if !notes.is_empty() {
    Ok(notes)
  }
  else{
    eprintln!("Erreur lors de la lecture des notes mises à jour");
    Err("Erreur lors de la lecture des notes mises à jour".to_string())
  }
}

// Fonction pour lire les notes depuis le fichier JSON
fn read_notes() -> Result<Vec<Note>, String> {
  let file_contents = fs::read_to_string("notes.json")
      .map_err(|err| err.to_string())?; // Convertir l'erreur en String

  let notes: Vec<Note> = serde_json::from_str(&file_contents)
      .map_err(|err| err.to_string())?; // Convertir l'erreur en String

  Ok(notes)
}

// Fonction pour écrire les notes dans le fichier JSON
fn write_notes(notes: &[Note]) -> Result<(), Error> {
  let notes_json = serde_json::to_string(notes)?;
  match fs::write("notes.json", notes_json) {
    Ok(_) => (),
    Err(e) => eprintln!("Erreur lors de l'écriture des notes : {}", e),
  }
  Ok(())
}

#[tauri::command]
fn update_note(id: String, newTitle: String, new_content: String) {
  // Convertir l'ID de String en i32
  let id_i32: Result<i32, _> = id.parse();

  // Vérifier si la conversion a réussi
  match id_i32 {
    Ok(id_i32) => {
      // Maintenant id_i32 est de type i32, vous pouvez l'utiliser
      // Appeler la fonction pour lire les notes
      let notes_result = read_notes();

      // Vérifier si la lecture des notes a réussi
      if let Ok(mut notes) = notes_result {
        // Rechercher la note correspondant à l'ID donné
        if let Some(note) = notes.iter_mut().find(|note| note.id == id_i32) {
          // Mettre à jour le titre et le contenu de la note
          note.title = newTitle;
          note.content = new_content;
          // Écrire les notes mises à jour dans le fichier
          if let Err(err) = write_notes(&notes) {
            println!("Failed to write updated notes: {}", err);
          }
        } else {
          println!("Note with ID {} not found", id_i32);
        }
      } else {
        println!("Error reading notes");
      }
    }
    Err(_) => {
      println!("Failed to parse ID as i32");
    }
  }
}

#[tauri::command]
fn delete_note(id: String) {
  // Convertir l'ID de String en i32
  let id_i32: Result<i32, _> = id.parse();

  // Vérifier si la conversion a réussi
  match id_i32 {
    Ok(id_i32) => {
      // Maintenant id_i32 est de type i32, vous pouvez l'utiliser
      // Appeler la fonction pour lire les notes
      let notes_result = read_notes();

      // Vérifier si la lecture des notes a réussi
      if let Ok(mut notes) = notes_result {
        // Trouver l'index de la note avec l'ID donné
        if let Some(index) = notes.iter().position(|note| note.id == id_i32) {
          // Supprimer la note de la liste
          notes.remove(index);
          // Écrire les notes mises à jour dans le fichier
          if let Err(err) = write_notes(&notes) {
            println!("Failed to write updated notes: {}", err);
          }
        } else {
          println!("Note with ID {} not found", id_i32);
        }
      } else {
        println!("Error reading notes");
      }
    }
    Err(_) => {
      println!("Failed to parse ID as i32");
    }
  }
}

fn init_db() -> Result<()> {
  let conn = Connection::open("notes.db")?;
  conn.execute("CREATE TABLE IF NOT EXISTS notes ( id INTEGER PRIMARY KEY,title TEXT NOT NULL, content TEXT NOT NULL, date DATE )",[],)?;
  Ok(())
}

#[tauri::command]
fn create_note_sqlite(title: &str, content: &str) -> bool {
  match create_note_sqlite_inner(title, content) {
    Ok(rows_affected) => rows_affected == 1,
    Err(_) => false, // Gérer l'erreur ici, par exemple journaliser l'erreur
  }
}

fn create_note_sqlite_inner(title: &str, content: &str) -> Result<usize> {
  let conn = Connection::open("notes.db")?;
  let date = Utc::now();
  let date_str = date.format("%d/%m/%Y").to_string();
  let rows_affected = conn.execute(
    "INSERT INTO notes (title, content, date) VALUES (?1, ?2, ?3)",
    params![title, content, date_str],
  )?;

  Ok(rows_affected)
}



fn read_notes_sql(conn: &Connection) -> Result<Vec<(i64, String, String,String)>, String> {
  let mut stmt = conn.prepare("SELECT id, title, content,date FROM notes")
      .map_err(|err| format!("Erreur lors de la préparation de la requête SQL : {}", err))?;

  let note_iter = stmt.query_map([], |row| {
    Ok((
      row.get(0)?,
      row.get(1)?,
      row.get(2)?,
      row.get(3)?,
    ))
  }).map_err(|err| format!("Erreur lors de l'exécution de la requête SQL : {}", err))?;

  note_iter.collect::<Result<Vec<_>, _>>()
      .map_err(|err| format!("Erreur lors de la collecte des résultats : {}", err))
}

#[tauri::command]
fn get_notes() -> Result<Vec<Note>, String> {
  // Connexion à la base de données
  let conn = Connection::open("notes.db")
      .map_err(|err| format!("Erreur lors de l'ouverture de la connexion à la base de données : {}", err))?;

  // Appel de la fonction read_notes_sql pour récupérer les notes
  let note_tuples = read_notes_sql(&conn)
      .map_err(|err| format!("Erreur lors de la récupération des notes depuis la base de données : {}", err))?;

  // Convertir les tuples en instances de Note
  let notes: Vec<Note> = note_tuples
      .into_iter()
      .map(|(id, title, content,date)| Note {  id: id.try_into().unwrap(), title, content,date })
      .collect();

  // Renvoi des notes au code JavaScript
  Ok(notes)
}

#[tauri::command]
fn update_note_sql(title: String, content: String, id: i64) -> bool {
  println!("update rust");
  let conn = Connection::open("notes.db")
      .expect("Erreur lors de l'ouverture de la connexion à la base de données");

  match update_note_sql_inner(&conn, id, &title, &content) {
    Ok(rows_affected) => rows_affected == 1,
    Err(_) => false, // Gérer l'erreur ici, par exemple journaliser l'erreur
  }
}

fn update_note_sql_inner(conn: &Connection, id: i64, title: &str, content: &str) -> Result<usize, rusqlite::Error> {
  let rows_affected = conn.execute(
    "UPDATE notes SET title = ?1, content = ?2 WHERE id = ?3",
    params![title, content, id],
  )?;
  Ok(rows_affected)
}


#[tauri::command]
fn delete_note_sql(id: i64) -> bool {
  let conn = Connection::open("notes.db")
      .expect("Erreur lors de l'ouverture de la connexion à la base de données");

  match delete_note_inner(&conn, id) {
    Ok(rows_affected) => rows_affected == 1,
    Err(_) => false, // Gérer l'erreur ici, par exemple journaliser l'erreur
  }
}

fn delete_note_inner(conn: &Connection, id: i64) -> Result<usize, rusqlite::Error> {
  conn.execute("DELETE FROM notes WHERE id = ?1", params![id])
}




fn main() -> Result<()> {
  init_db()?;
  let conn = Connection::open("notes.db")?;

  // Appelez la fonction pour lire les notes
  match read_notes_sql(&conn) {
    Ok(notes) => {
      // Afficher les notes dans le terminal
      println!("Notes lues depuis la base de données :");
      for note in notes {
        println!("ID: {}, Titre: {}, Contenu: {}", note.0, note.1, note.2);
      }
    }
    Err(err) => {
      println!("Erreur lors de la lecture des notes depuis la base de données : {}", err);
    }
  }

  // Lire les notes depuis la base de données

  tauri::Builder::default()
      .invoke_handler(tauri::generate_handler![create_note, fetch_notes, update_note, delete_note, create_note_sqlite,get_notes,update_note_sql,delete_note_sql])
      .run(tauri::generate_context!())
      .expect("erreur lors de l'exécution de l'application Tauri");

  Ok(())
}
