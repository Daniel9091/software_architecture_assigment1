pub fn bottom_nav_html() -> String {
    r#"
    <nav style="position:fixed;bottom:0;left:0;right:0;height:56px;background:#222;color:#fff;display:flex;justify-content:space-around;align-items:center;">
        <a href="/authors" style="color:#fff;text-decoration:none;padding:8px 16px;">Autores</a>
        <a href="/books"   style="color:#fff;text-decoration:none;padding:8px 16px;">Libros</a>
        <a href="/tables"  style="color:#fff;text-decoration:none;padding:8px 16px;">Tablas</a>
    </nav>
    "#
    .to_string()
}

pub fn render_page(title: &str, body: &str) -> String {
    format!(
        r#"<!doctype html>
<html lang="es">
<head>
  <meta charset="utf-8"/>
  <meta name="viewport" content="width=device-width,initial-scale=1"/>
  <title>{}</title>
  <style>
    body {{ margin:0 0 76px 0; font-family:Arial,Helvetica,sans-serif; padding:16px; }}
    .container {{ max-width:900px; margin:0 auto; }}
    ul {{ padding-left:18px; }}
    .loading {{ color:#666; }}
  </style>
</head>
<body>
  <div class="container">
    <h1>{}</h1>
    {}
  </div>
  {}
</body>
</html>"#,
        title,
        title,
        body,
        bottom_nav_html()
    )
}
