// For the fonts make sure you add this into the css:
// @font-face {{
//  font-family: "Public Sans";
//  src: url("/src/assets/public-sans.ttf");
// }}

pub fn header() -> String {
  return r#"
    <style>
      .header_format {
        display: flex; 
        padding-left: 10rem;
      }

      @media (max-width: 860px) {
        .header_format {
          padding-left: 5rem;
        }
      }

      @media (max-width: 480px) {
        .header_format {
          padding-left: 1rem;
        }
      }
    </style>
    <div style="display: flex; align-items: center; width: 100%; height: 65px; background-color: #9E5A40;">
      <div class="header_format">
        <a href="/" style="text-decoration: none;">
          <div style="display: flex; gap: 10px; align-items: center;">
            <img src="/src/assets/rust_logo.png" style="user-select: none;" width="50px" height="50px">
            <h2 style="font-size: 22px; font-family: Public Sans; color: #ffffff; user-select: none;">Rustiy</h2>
          </div>
        </a>
      </div>
    </div>
  "#.to_string()
}
