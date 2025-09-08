use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use serde_json::json;
use opensearch::SearchParts;
use crate::opensearch_client::os_client;

#[derive(Serialize)]
struct HitItem {
    id: Option<i32>,
    title: Option<String>,
    author: Option<String>,
    description: Option<String>,
    genres: Vec<String>,
    published_year: Option<i32>,
    rating: Option<f32>,
    score: Option<f32>,
    highlight: Option<serde_json::Value>,
}

#[derive(Serialize)]
struct SearchResponse {
    total: u64,
    items: Vec<HitItem>,
}

#[derive(Deserialize)]
struct OSResp {
    hits: Hits,
}
#[derive(Deserialize)]
struct Hits {
    total: serde_json::Value,
    hits: Vec<OSHit>,
}
#[derive(Deserialize)]
struct OSHit {
    #[serde(default)]
    _id: String,
    #[serde(default)]
    _score: Option<f32>,
    #[serde(default)]
    _source: serde_json::Value,
    #[serde(default)]
    highlight: Option<serde_json::Value>,
}

#[rocket::get("/search?<q>&<from>&<size>")]
pub async fn search(q: String, from: Option<u32>, size: Option<u32>) -> Json<SearchResponse> {
    let idx = std::env::var("OS_INDEX_BOOKS").unwrap_or_else(|_| "books".into());

    if q.trim().is_empty() {
        return Json(SearchResponse { total: 0, items: vec![] });
    }

    let body = json!({
        "from": from.unwrap_or(0),
        "size": size.unwrap_or(10),
        "_source": ["id","title","author","description","genres","rating","published_year"],
        "query": {
            "bool": {
            "must": [
                {
                "multi_match": {
                    "query": q,
                    "type": "cross_fields",
                    "operator": "and",
                    "fields": ["title^4","author^3","description"]
                }
                }
            ],
            "should": [
                {
                "multi_match": {
                    "query": q,
                    "type": "best_fields",
                    "fields": ["title^3","author^2","description"],
                    "fuzziness": "AUTO"
                }
                }
            ],
            "minimum_should_match": 0
            }
        },
        "highlight": {
            "fields": { "title": {}, "author": {}, "description": {} },
            "pre_tags": ["<mark>"], "post_tags": ["</mark>"]
        }
        });


    let resp = os_client()
        .search(SearchParts::Index(&[idx.as_str()]))
        .body(body)
        .send().await;

    // Por defecto, respuesta vacía si hay error
    let mut out = SearchResponse { total: 0, items: vec![] };

    if let Ok(ok) = resp {
        if let Ok(v) = ok.json::<OSResp>().await {
            // total puede venir como objeto {"value":N} o número
            let total = v.hits.total.get("value")
                .and_then(|x| x.as_u64())
                .or_else(|| v.hits.total.as_u64())
                .unwrap_or(0);
            out.total = total;

            for h in v.hits.hits {
                let s = &h._source;

                // id puede venir como string o número; si no está, probamos _id numérico
                let id = s.get("id")
                    .and_then(|x| x.as_i64())
                    .map(|n| n as i32)
                    .or_else(|| s.get("id").and_then(|x| x.as_str())?.parse::<i32>().ok())
                    .or_else(|| h._id.parse::<i32>().ok());

                let title = s.get("title").and_then(|x| x.as_str()).map(|s| s.to_string());
                let author = s.get("author").and_then(|x| x.as_str()).map(|s| s.to_string());
                let description = s.get("description").and_then(|x| x.as_str()).map(|s| s.to_string());
                let published_year = s.get("published_year").and_then(|x| x.as_i64()).map(|n| n as i32);
                let rating = s.get("rating").and_then(|x| x.as_f64()).map(|f| f as f32);
                let genres = s.get("genres")
                    .and_then(|x| x.as_array())
                    .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect::<Vec<_>>())
                    .unwrap_or_default();

                out.items.push(HitItem {
                    id,
                    title,
                    author,
                    description,
                    genres,
                    published_year,
                    rating,
                    score: h._score,
                    highlight: h.highlight,
                });
            }
        }
    }

    Json(out)
}
