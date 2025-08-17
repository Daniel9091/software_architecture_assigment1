// src/repository/mod.rs
pub mod init;
pub mod authors;
pub mod books;
pub mod reviews;
pub mod sales;
pub mod dashboard;

// Re-exports para mantener el API anterior:
pub use authors::{get_all_authors, get_author_by_id, create_author, update_author, delete_author};
pub use books::{get_all_books, get_book_by_id, create_book, update_book, delete_book};
pub use reviews::{get_reviews_by_book, create_review, update_review, delete_review};
pub use sales::{get_yearly_sales_by_book, create_yearly_sales, update_yearly_sales, delete_yearly_sales};
pub use dashboard::get_dashboard_stats;
