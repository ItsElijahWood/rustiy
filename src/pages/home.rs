use crate::components::header::header;

pub fn home() -> String {
  let header = header();

  return format!(r#"
    <!DOCTYPE html>
    <html lang="en">
      <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <link rel="icon" href="/src/assets/rust_logo.png" type="image/x-icon">
        <title>Rustiy</title>
      </head>
      <body>
        <style>
          body {{
            margin: 0;
            background-color: #524c4c;
          }}

          .format {{
            display: flex; 
            justify-content: center; 
            padding-left: 10rem; 
            padding-right: 10rem;
            padding-top: 3rem;
          }}

          .text-format {{
            font-family: Public Sans; 
            font-size: 22px; 
            color: white;
          }}

          @media (max-width: 860px) {{
            .text-format {{
              max-width: 480px;
            }}
          }}

          @media (max-width: 480px) {{
            .format {{
              padding-left: 0; 
              padding-right: 0;
            }}
            .text-format {{
              max-width: 380px;
            }}
          }}

          @font-face {{
            font-family: "Public Sans";
            src: url("/src/assets/public-sans.ttf");
          }}
        </style>
        {}
        <div class="format">
          <div style="display: flex; flex-direction: column; align-items: center; text-align: center;">
            <img style="user-select: none;" src="/src/assets/rust_crab.png" width="350px" height="250px">
            <p style="font-family: Public Sans; font-size: 23px; color: white; max-width: 300px;">Your quick start to building awseome Rust projects.</p>
            <br>
            <div style="text-align: left;">
              <p class="text-format">We will guide you through the entire process, from installing Rust to building your own web server!  All the guides on this website are made by <a href="https://github.com/itselijahwood" style="color: #00b0ff">ItsElijahWood</a> from passed experience of C, JavaScript and PHP. You can access all the code from GitHub at <a href="https://github.com/itselijahwood" style="color: #00b0ff">ItsElijahWood</a>.</p>
              <p class="text-format">This website is built entirely in standard vanilla Rust! No frameworks no librarys.</p>
            </div>
          </div>
        </div>
      </body>
    </html>
  "#, header)
}
