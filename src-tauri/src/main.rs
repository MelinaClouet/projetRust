use std::io::Error;
use std::fs;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize}; // Importation des traits Serialize et Deserialize
use rusqlite::{params, Connection, Result};


#[derive(Debug, Serialize, Deserialize, Clone)]
struct Note {
  id: i32,
  title: String,
  description: String,
  date: String,
}

// Fonction pour créer une nouvelle note et la stocker dans un fichier JSON
#[tauri::command]
fn create_note(title: String, description: String) -> Result<Vec<Note>, String> {
  // Créer la nouvelle note
  let id = generate_unique_id();
  let date = Utc::now();
  let date_str = date.format("%d/%m/%Y").to_string();

  let new_note = Note {
    id,
    title,
    description,
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
fn update_note(id: String, newTitle: String, new_description: String) {
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
          // Mettre à jour le titre et la description de la note
          note.title = newTitle;
          note.description = new_description;
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
  // Ouvrir la connexion à la base de données ou la créer si elle n'existe pas
  let conn = Connection::open("notes.db")?;

  // Exécuter la requête SQL pour créer la table "notes"
  conn.execute(
    "CREATE TABLE IF NOT EXISTS notes (
            id INTEGER PRIMARY KEY,
            title TEXT NOT NULL,
            content TEXT NOT NULL,
            date TEXT
        )",
    [],
  )?;

  // Retourner un résultat Ok() si tout s'est bien passé
  Ok(())
}


fn create_note_sqlite(conn: &Connection, title: &str, content: &str) -> Result<(), rusqlite::Error> {
  let now = chrono::Utc::now();
  let date_str = now.to_rfc3339(); // Convertir la date en format texte

  conn.execute(
    "INSERT INTO notes (title, content, date) VALUES (?1, ?2, ?3)",
    params![title, content, date_str],
  )?;
  Ok(())
}

fn main() {
  init_db();
  tauri::Builder::default()
      .invoke_handler(tauri::generate_handler![create_note, fetch_notes,update_note,delete_note,create_note_sqlite])
      .run(tauri::generate_context!())
      .expect("erreur lors de l'exécution de l'application Tauri");
}
