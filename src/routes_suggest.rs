use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use serde_json::json;
use opensearch::SearchParts;
use crate::opensearch_client::os_client;

#[derive(Serialize)]
struct SuggestResponse {
    suggestions: Vec<String>,
}

#[derive(Deserialize)]
struct OSugg {
    suggest: serde_json::Value,
}

#[rocket::get("/suggest?<q>")]
pub async fn suggest(q: String) -> Json<SuggestResponse> {
    let idx = std::env::var("OS_INDEX_BOOKS").unwrap_or_else(|_| "books".into());

    // Sin query => vacío
    if q.trim().is_empty() {
        return Json(SuggestResponse { suggestions: vec![] });
    }

    let body = json!({
      "size": 0,
      "suggest": {
        "s": {
          "prefix": q,
          "completion": { "field": "suggest", "skip_duplicates": true, "fuzzy": { "fuzziness": 1 }, "size": 10 }
        }
      }
    });

    let resp = os_client()
        .search(SearchParts::Index(&[idx.as_str()]))
        .body(body)
        .send().await;

    let mut out: Vec<String> = Vec::new();

    if let Ok(ok) = resp {
        if let Ok(v) = ok.json::<OSugg>().await {
            if let Some(arr) = v.suggest
                .get("s").and_then(|s| s.get(0))
                .and_then(|z| z.get("options")).and_then(|o| o.as_array()) {
                for o in arr {
                    if let Some(t) = o.get("text").and_then(|t| t.as_str()) {
                        out.push(t.to_string());
                    }
                }
            }
        }
    }

    Json(SuggestResponse { suggestions: out })
}
