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

      .menu {
        width: 40px;
        height: 40px;
      }

      .mobile-menu {
        position: fixed;
        visibility: hidden;
        background-color: #9E5A40;
        right: 0;
        z-index: 999;
        opacity: 0;
        transform: translateX(100%);
        transition: transform 0.5s ease, opacity 1s ease; 
      }

      .mobile-menu.active {
        visibility: visible;
        display: flex;
        position: absolute !important;
        width: 300px;
        height: 100vh;
        opacity: 1 !important;
        transform: translateX(0) !important; 
      }

      @media (max-width: 860px) {
        .header_format {
          padding-left: 4.5rem;
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
        .menu {
          width: 30px;
          height: 30px;
        }
        .menu-div {
          top: 18px;
        }
        .mobile-menu.active {
          width: 100% !important;
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
        <div id="menu-div" class="menu-div">
          <img class="menu" src="/src/assets/menu_icon.png">
        </div>
      </div>
    </div>
    <div class="mobile-menu">
      <div style="display: flex; flex-direction: column; justify-content: center; align-items: center; gap: 20px; width: 100%; height: 90vh;">
        <a style="color: white; font-family: Public Sans; font-size: 23px; text-decoration: none; color: #cbcbcb;" href="/projects">Projects</a>
        <a style="color: white; font-family: Public Sans; font-size: 23px; text-decoration: none; color: #cbcbcb;" href="/about">About</a>
      </div>
    </div>
    <script>
      document.addEventListener('DOMContentLoaded', () => {
        const menuToggle = document.getElementById('menu-div');
        if (menuToggle) {
          menuToggle.addEventListener('click', () => {
            const menu = document.querySelector('.mobile-menu');
            if (menu) {
              menu.classList.toggle('active');
            }
          });
        }
      });
    </script>
  "#.to_string()
}
