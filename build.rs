// Change this to your icon's location
const ICON: &str = "assets/icons/icon.ico";

fn main() {
  if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
    let mut res = winresource::WindowsResource::new();
    res.set_icon(ICON);
    res.compile().unwrap();
  }
}
