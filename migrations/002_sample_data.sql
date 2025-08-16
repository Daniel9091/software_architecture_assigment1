-- Insertar autores de ejemplo
INSERT INTO authors (name, birth_date, country, description) VALUES
('Gabriel García Márquez', '1927-03-06', 'Colombia', 'Escritor, guionista, editor y periodista colombiano. Premio Nobel de Literatura en 1982.'),
('Jorge Luis Borges', '1899-08-24', 'Argentina', 'Escritor, poeta, ensayista y traductor argentino, una de las figuras más importantes de la literatura universal.'),
('Isabel Allende', '1942-08-02', 'Chile', 'Escritora chilena, considerada una de las autoras más leídas del mundo de habla hispana.'),
('Mario Vargas Llosa', '1936-03-28', 'Perú', 'Escritor peruano, Premio Nobel de Literatura en 2010.'),
('Pablo Neruda', '1904-07-12', 'Chile', 'Poeta chileno, Premio Nobel de Literatura en 1971.');

-- Insertar libros de ejemplo
INSERT INTO books (title, summary, publication_date, sales_count, author_id) VALUES
('Cien años de soledad', 'Crónica de la familia Buendía a lo largo de siete generaciones en el pueblo ficticio de Macondo.', '1967-06-05', 50000000, 1),
('El Aleph', 'Colección de cuentos que exploran temas como el infinito, el tiempo y la realidad.', '1949-06-30', 2000000, 2),
('La casa de los espíritus', 'Historia de la familia Trueba a lo largo de cuatro generaciones en Chile.', '1982-01-01', 15000000, 3),
('La ciudad y los perros', 'Novela que narra la vida de los cadetes en el Colegio Militar Leoncio Prado.', '1963-01-01', 8000000, 4),
('Veinte poemas de amor y una canción desesperada', 'Colección de poemas de amor que se ha convertido en una de las obras más famosas del autor.', '1924-06-15', 25000000, 5);

-- Insertar reseñas de ejemplo
INSERT INTO reviews (book_id, review_text, rating, positive_votes) VALUES
(1, 'Una obra maestra de la literatura latinoamericana. La narrativa mágica y la riqueza de personajes la convierten en una lectura imprescindible.', 5, 1250),
(1, 'Excelente novela, aunque puede ser compleja de seguir en algunos momentos. La prosa es magnífica.', 4, 890),
(2, 'Borges demuestra su genio en cada cuento. Una obra que desafía la percepción de la realidad.', 5, 756),
(3, 'Historia conmovedora y bien escrita. Los personajes femeninos están muy bien desarrollados.', 4, 634),
(4, 'Novela cruda y realista sobre la vida militar. Excelente retrato de la sociedad peruana.', 4, 445),
(5, 'Poesía pura y emotiva. Cada poema es una joya literaria.', 5, 1120);

-- Insertar ventas por año de ejemplo
INSERT INTO yearly_sales (book_id, year, sales) VALUES
(1, 2020, 150000),
(1, 2021, 180000),
(1, 2022, 165000),
(1, 2023, 190000),
(2, 2020, 45000),
(2, 2021, 52000),
(2, 2022, 48000),
(2, 2023, 55000),
(3, 2020, 80000),
(3, 2021, 95000),
(3, 2022, 87000),
(3, 2023, 102000),
(4, 2020, 35000),
(4, 2021, 42000),
(4, 2022, 38000),
(4, 2023, 45000),
(5, 2020, 120000),
(5, 2021, 135000),
(5, 2022, 128000),
(5, 2023, 142000); 