use rocket::response::content::RawHtml;
use rocket::State;
use crate::db;
use crate::Db;

// Ruta mínima: consulta estadísticas rápidas para comprobar la BD y modelos.
#[get("/")]
pub async fn index(pool: &State<Db>) -> RawHtml<String> {
    match db::get_dashboard_stats(&pool.0).await {
        Ok(stats) => {
            let pretty = serde_json::to_string_pretty(&stats).unwrap_or_else(|_| "{}".to_string());
            RawHtml(format!("<pre>{}</pre>", pretty))
        }
        Err(e) => RawHtml(format!("Error al consultar la base de datos: {}", e)),
    }
}
