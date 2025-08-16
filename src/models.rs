use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Author {
    pub id: Option<i32>,
    pub name: String,
    pub birth_date: String,
    pub country: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Book {
    pub id: Option<i32>,
    pub title: String,
    pub summary: Option<String>,
    pub publication_date: String,
    pub sales_count: i32,
    pub author_id: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BookWithAuthor {
    pub id: Option<i32>,
    pub title: String,
    pub summary: Option<String>,
    pub publication_date: String,
    pub sales_count: i32,
    pub author: Author,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Review {
    pub id: Option<i32>,
    pub book_id: i32,
    pub review_text: String,
    pub rating: i32,
    pub positive_votes: i32,
    pub created_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReviewWithBook {
    pub id: Option<i32>,
    pub book_id: i32,
    pub book_title: String,
    pub review_text: String,
    pub rating: i32,
    pub positive_votes: i32,
    pub created_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct YearlySales {
    pub id: Option<i32>,
    pub book_id: i32,
    pub year: i32,
    pub sales: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct YearlySalesWithBook {
    pub id: Option<i32>,
    pub book_id: i32,
    pub book_title: String,
    pub year: i32,
    pub sales: i32,
}

// DTOs para crear/actualizar entidades
#[derive(Debug, Deserialize)]
pub struct CreateAuthor {
    pub name: String,
    pub birth_date: String,
    pub country: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateBook {
    pub title: String,
    pub summary: Option<String>,
    pub publication_date: String,
    pub author_id: i32,
}

#[derive(Debug, Deserialize)]
pub struct CreateReview {
    pub book_id: i32,
    pub review_text: String,
    pub rating: i32,
}

#[derive(Debug, Deserialize)]
pub struct CreateYearlySales {
    pub book_id: i32,
    pub year: i32,
    pub sales: i32,
}

// DTOs para actualizar entidades
#[derive(Debug, Deserialize)]
pub struct UpdateAuthor {
    pub name: Option<String>,
    pub birth_date: Option<String>,
    pub country: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateBook {
    pub title: Option<String>,
    pub summary: Option<String>,
    pub publication_date: Option<String>,
    pub author_id: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateReview {
    pub book_id: Option<i32>,
    pub review_text: Option<String>,
    pub rating: Option<i32>,
    pub positive_votes: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateYearlySales {
    pub book_id: Option<i32>,
    pub year: Option<i32>,
    pub sales: Option<i32>,
}

// Respuestas de la API
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: String,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: "Operación exitosa".to_string(),
        }
    }

    pub fn error(message: &str) -> Self {
        Self {
            success: false,
            data: None,
            message: message.to_string(),
        }
    }
} 