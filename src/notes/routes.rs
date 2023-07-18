use super::structs::Note;
use actix_web::{get, put, web, HttpResponse, Responder};
use futures::stream::TryStreamExt;
use mongodb::{bson::doc, options::FindOptions, Client, Collection};
use uuid::Uuid;

#[get("/all")]
pub async fn handle_all_notes(client: web::Data<Client>) -> impl Responder {
    let notes = fetch_all_notes(client).await;

    return HttpResponse::Ok().body(notes.unwrap());
}

#[put("/create")]
pub async fn handle_create_note(client: web::Data<Client>, req_body: String) -> impl Responder {
    let result = serde_json::from_str::<Note>(&req_body);
    if result.is_err() {
        return HttpResponse::BadRequest().body("Bad Request")
    }

    let note = result.unwrap();

    let collection: Collection<Note> = client.database("notes").collection("notes");
    let _ = collection
        .insert_one(
            Note {
                _id: Uuid::new_v4().to_string(),
                index: note.index,
                title: note.title,
                text: note.text,
            },
            None,
        )
        .await;

    let notes = fetch_all_notes(client).await;

    return HttpResponse::Ok().body(notes.unwrap());
}

async fn fetch_all_notes(client: web::Data<Client>) -> Result<String, serde_json::Error> {
    let collection: Collection<Note> = client.database("notes").collection("notes");

    let result = collection
        .find(
            doc! {},
            FindOptions::builder().sort(doc! { "index": 1 }).build(),
        )
        .await;

    let mut cursor: mongodb::Cursor<Note> = result.expect("failed to extract notes from Result");

    let mut notes: Vec<Note> = Vec::new();
    while let Ok(Some(note)) = cursor.try_next().await {
        notes.push(note);
    }

    let json = serde_json::to_string(&notes);
    return json;
}
