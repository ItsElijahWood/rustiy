use crate::components::header::header;

pub fn projects() -> String {
  let header = header();

  return format!(r#"
    <!DOCTYPE html>
    <html lang="en">
      <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <link rel="icon" href="/src/assets/rust_logo.png" type="image/x-icon">
        <title>Rustiy Projects</title>
      </head>
      <body>
        <style>
          body {{
            margin: 0;
            background-color: #524c4c;
          }}

          @font-face {{
            font-family: "Public Sans";
            src: url("/src/assets/public-sans.ttf") format("truetype");
          }}
        </style>
        {}
      </body>
    </html>
  "#, header);
}
