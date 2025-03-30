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
        padding-left: 9rem;
      }

      .button-div {
        display: flex;
        position: absolute;
        gap: 30px;
        right: 0px;
        top: 19px;
        align-items: center;
        width: 500px;
      }

      .menu-div {
        display: none;
        position: absolute;
        right: 15px;
        top: 13px;
      }

      @media (max-width: 860px) {
        .header_format {
          padding-left: 9.5rem;
        }
        .button-div {
          display: none;
        }
        .menu-div {
          display: flex;
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
        <div class="button-div">
          <a style="color: white; font-family: Public Sans; font-size: 18px; text-decoration: none; color: #cbcbcb;" href="/projects">Projects</a>
          <a style="color: white; font-family: Public Sans; font-size: 18px; text-decoration: none; color: #cbcbcb;" href="/about">About</a>
        </div>
        <div class="menu-div">
          <img src="/src/assets/menu_icon.png" width="40px" height="40px">
        </div>
      </div>
    </div>
  "#.to_string()
}
