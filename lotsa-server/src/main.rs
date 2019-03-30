use actix_web::{server, App, fs};

pub fn main() {
  server::new(|| {
    let root = fs::StaticFiles::new("www/dis")
      .expect("find www/dist directory generated by webpack") // TODO
      .index_file("index.html");
    App::new().handler("/", root)
  })
    .bind("127.0.0.1:80")
    .expect("bind to open port") // TODO
    .run()
}
