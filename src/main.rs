use minifb::{Window, WindowOptions};

fn main() {
  let width = 640;
  let height = 480;
  let mut buffer = vec![0; width * height];
  let mut window = Window::new("Rust Window", width, height, WindowOptions::default())
    .unwrap_or_else(|e| {
      panic!("{}", e);
    });

  while window.is_open() && !window.is_key_down(minifb::Key::Escape) {
    for i in buffer.iter_mut() {
      *i = 0x000000;
    }
    window.update_with_buffer(&buffer, width, height).unwrap();
  }
}
