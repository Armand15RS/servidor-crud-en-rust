pub async fn edit_note_handler(
    Path(id): Path<String>,
    State(data): State<Arc<AppState>>,
    Json(body): Json<UpdateNoteSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    
    let query_result = sqlx::query_as!(
        NoteModel,
        r#"SELECT * FROM notes WHERE id = ?"#,
        &id
    )
    .fetch_one(&data.db)
    .await;

    
    let note = match query_result {
        Ok(note) => note,
        Err(sqlx::Error::RowNotFound) => {
            let error_response = serde_json::json!({
                "status": "error",
                "message": format!("Note with ID: {} not found", id)
            });
            return Err((StatusCode::NOT_FOUND, Json(error_response)));
        }
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "error",
                    "message": format!("{:?}", e)
                })),
            ));
        }
    };

    
    let is_published = body.is_published.unwrap_or(note.is_published != 0);
    let i8_is_published = is_published as i8;

    
    let update_result =
        sqlx::query(r#"UPDATE notes SET title = ?, content = ?, is_published = ? WHERE id = ?"#)
            .bind(&body.title.unwrap_or_else(|| note.title))
            .bind(&body.content.unwrap_or_else(|| note.content))
            .bind(i8_is_published)
            .bind(&id)
            .execute(&data.db)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "status": "error",
                        "message": format!("{:?}", e)
                    })),
                )
            })?;

    
    if update_result.rows_affected() == 0 {
        let error_response = serde_json::json!({
            "status": "error",
            "message": format!("Note with ID: {} not found", id)
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    
    let updated_note = sqlx::query_as!(
        NoteModel,
        r#"SELECT * FROM notes WHERE id = ?"#,
        &id
    )
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
            "note": to_note_response(&updated_note)
        })
    });

    Ok(Json(note_response))
}