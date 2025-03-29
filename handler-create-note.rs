pub async fn create_note_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreateNoteSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    
    let id = uuid::Uuid::new_v4().to_string();
    let query_result = sqlx::query(r#"INSERT INTO notes (id, title, content) VALUES (?, ?, ?)"#)
        .bind(&id)
        .bind(&body.title)
        .bind(&body.content)
        .execute(&data.db)
        .await
        .map_err(|err: sqlx::Error| err.to_string());

    
    if let Err(err) = query_result {
        if err.contains("Duplicate entry") {
            let error_response = serde_json::json!({
                "status": "error",
                "message": "Note already exists",
            });
            return Err((StatusCode::CONFLICT, Json(error_response)));
        }

        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"status": "error","message": format!("{:?}", err)})),
        ));
    }

    
    let note = sqlx::query_as!(NoteModel, r#"SELECT * FROM notes WHERE id = ?"#, &id)
        .fetch_one(&data.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error","message": format!("{:?}", e)})),
            )
        })?;

    let note_response = serde_json::json!({
            "status": "success",
            "data": serde_json::json!({
                "note": to_note_response(&note)
        })
    });

    Ok(Json(note_response))
}