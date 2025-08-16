-- Crear tabla de autores
CREATE TABLE IF NOT EXISTS authors (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    birth_date DATE NOT NULL,
    country TEXT NOT NULL,
    description TEXT
);

-- Crear tabla de libros
CREATE TABLE IF NOT EXISTS books (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    summary TEXT,
    publication_date DATE NOT NULL,
    sales_count INTEGER DEFAULT 0,
    author_id INTEGER NOT NULL,
    FOREIGN KEY (author_id) REFERENCES authors(id) ON DELETE CASCADE
);

-- Crear tabla de reseñas
CREATE TABLE IF NOT EXISTS reviews (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    book_id INTEGER NOT NULL,
    review_text TEXT NOT NULL,
    rating INTEGER NOT NULL CHECK (rating >= 1 AND rating <= 5),
    positive_votes INTEGER DEFAULT 0,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (book_id) REFERENCES books(id) ON DELETE CASCADE
);

-- Crear tabla de ventas por año
CREATE TABLE IF NOT EXISTS yearly_sales (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    book_id INTEGER NOT NULL,
    year INTEGER NOT NULL,
    sales INTEGER NOT NULL,
    FOREIGN KEY (book_id) REFERENCES books(id) ON DELETE CASCADE,
    UNIQUE(book_id, year)
);

-- Crear índices para mejorar el rendimiento
CREATE INDEX IF NOT EXISTS idx_books_author ON books(author_id);
CREATE INDEX IF NOT EXISTS idx_reviews_book ON reviews(book_id);
CREATE INDEX IF NOT EXISTS idx_yearly_sales_book ON yearly_sales(book_id);
CREATE INDEX IF NOT EXISTS idx_yearly_sales_year ON yearly_sales(year); 