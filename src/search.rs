use anyhow::{anyhow, Result};
use reqwest::{Client, Url};
use serde::Deserialize;
use serde_json::{json, Value};
use std::env;

#[derive(Clone)]
pub struct Search {
    enabled: bool,
    client: Client,
    base: String,         // https://localhost:9200  ó  https://opensearch:9200
    idx_books: String,    // books
    idx_reviews: String,  // reviews (por si lo usas luego)
    user: Option<String>,
    pass: Option<String>,
}

impl Search {
    /// Llama a esto una sola vez y guárdalo como State<> en Rocket.
    pub fn from_env() -> Result<Self> {
        let enabled = matches!(
            env::var("USE_OPENSEARCH")
                .unwrap_or_else(|_| "0".to_string())
                .to_ascii_lowercase()
                .as_str(),
            "1" | "true" | "yes" | "on"
        );

        // Valores por defecto: HTTPS + self-signed en DEV
        let base = env::var("OS_NODE").unwrap_or_else(|_| "https://localhost:9200".into());
        // Normaliza URL (quita trailing slash)
        let base = base.trim_end_matches('/').to_string();

        let idx_books   = env::var("OS_INDEX_BOOKS").unwrap_or_else(|_| "books".into());
        let idx_reviews = env::var("OS_INDEX_REVIEWS").unwrap_or_else(|_| "reviews".into());
        let user = env::var("OS_USER").ok();
        let pass = env::var("OS_PASS").ok();

        // Cliente que acepta cert self-signed (SOLO DEV). En prod, validar cert.
        let client = reqwest::Client::builder()
            .danger_accept_invalid_certs(true)
            .build()?;

        Ok(Self { enabled, client, base, idx_books, idx_reviews, user, pass })
    }

    fn auth(&self, rb: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        match (&self.user, &self.pass) {
            (Some(u), Some(p)) => rb.basic_auth(u, Some(p)),
            _ => rb,
        }
    }

    fn url_index_search(&self, index: &str) -> Result<Url> {
        let url = format!("{}/{}/_search", self.base, index);
        Ok(Url::parse(&url)?)
    }

    // === AUTOCOMPLETE: sugiere títulos (o frases) mientras escribes ===
    pub async fn suggest_titles(&self, prefix: &str, limit: u32) -> Result<Vec<String>> {
        if !self.enabled {
            // Modo local: sin OS → devolvemos vacío (o llama a tu lógica local si la tienes)
            return Ok(vec![]);
        }
        if prefix.trim().is_empty() { return Ok(vec![]); }

        let url = self.url_index_search(&self.idx_books)?;
        let body = json!({
            "size": 0,
            "suggest": {
                "s": {
                    "prefix": prefix,
                    "completion": {
                        "field": "suggest",
                        "skip_duplicates": true,
                        "fuzzy": { "fuzziness": 1 },
                        "size": limit
                    }
                }
            }
        });

        let rb = self.client.post(url).json(&body);
        let res = self.auth(rb).send().await?.error_for_status()?;
        let v: Value = res.json().await?;

        // Ruta: suggest.s[0].options[].text
        let mut out = Vec::new();
        if let Some(arr) = v.get("suggest").and_then(|s| s.get("s")).and_then(|s| s.get(0)).and_then(|s| s.get("options")).and_then(|o| o.as_array()) {
            for o in arr {
                if let Some(text) = o.get("text").and_then(|t| t.as_str()) {
                    out.push(text.to_string());
                }
            }
        }
        Ok(out)
    }

    // === BÚSQUEDA: devuelve IDs de libros para que luego hagas fetch a tu DB ===
    pub async fn search_book_ids(&self, q: &str, from: u32, size: u32) -> Result<Vec<i32>> {
        if !self.enabled {
            // Modo local: llama a tu búsqueda actual (si la tienes):
            // return self.search_book_ids_local(q, from, size).await;
            return Ok(vec![]);
        }
        if q.trim().is_empty() { return Ok(vec![]); }

        let url = self.url_index_search(&self.idx_books)?;
        let body = json!({
            "from": from, "size": size,
            "_source": ["id","book_id"], // por si tu _source tiene 'id' o 'book_id'
            "query": {
              "multi_match": {
                "query": q,
                "type": "best_fields",
                "fields": ["title^4","title.ac^5","author^3","author.ac^3","description"],
                "fuzziness": "AUTO",
                "operator": "and"
              }
            }
        });

        let rb = self.client.post(url).json(&body);
        let res = self.auth(rb).send().await?.error_for_status()?;
        let v: OSResp = res.json().await?;

        // Intentamos 3 caminos para sacar el ID entero:
        // 1) _source.book_id (i32)
        // 2) _source.id (string o número)
        // 3) _id (string numérica)
        let mut ids: Vec<i32> = Vec::new();
        for h in v.hits.hits.into_iter() {
            let mut push_id = |candidate: Option<i32>| {
                if let Some(id) = candidate {
                    if !ids.contains(&id) { ids.push(id); }
                }
            };
            // _source.book_id
            if let Some(id) = h._source.get("book_id").and_then(json_to_i32) {
                push_id(Some(id));
                continue;
            }
            // _source.id
            if let Some(id) = h._source.get("id").and_then(json_to_i32) {
                push_id(Some(id));
                continue;
            }
            // _id
            if let Ok(id) = h._id.parse::<i32>() {
                push_id(Some(id));
            }
        }
        Ok(ids)
    }
}

// Helpers

#[derive(Deserialize)]
struct OSResp {
    hits: OSHits,
}
#[derive(Deserialize)]
struct OSHits {
    hits: Vec<OSHit>,
}
#[derive(Deserialize)]
struct OSHit {
    #[serde(default)]
    _id: String,
    #[serde(default)]
    _source: Value,
}

fn json_to_i32(v: &Value) -> Option<i32> {
    if let Some(n) = v.as_i64() { return i32::try_from(n).ok(); }
    if let Some(s) = v.as_str() { return s.parse::<i32>().ok(); }
    None
}
