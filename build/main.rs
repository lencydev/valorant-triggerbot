use winres::WindowsResource;

fn main () {

  if cfg!(target_os = "windows") {

    WindowsResource::new().set_icon("assets/favicon.ico").compile().unwrap();
  }
}