pub async fn get_note_handler(
    Path(id): Path<String>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    
    let query_result = sqlx::query_as!(
        NoteModel,
        r#"SELECT * FROM notes WHERE id = ?"#,
        &id
    )
    .fetch_one(&data.db)
    .await;

    
    match query_result {
        Ok(note) => {
            let note_response = serde_json::json!({
                "status": "success",
                "data": serde_json::json!({
                    "note": to_note_response(&note)
                })
            });

            return Ok(Json(note_response));
        }
        Err(sqlx::Error::RowNotFound) => {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": format!("La nota con el ID: {} no encontrado", id)
            });
            return Err((StatusCode::NOT_FOUND, Json(error_response)));
        }
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error","message": format!("{:?}", e)})),
            ));
        }
    };
}
