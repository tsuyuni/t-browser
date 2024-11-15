use iced::widget::{button, column, text, Column};

#[derive(Default)]
struct Counter {
  value: i32,
}

#[derive(Debug, Clone)]
enum Message {
  Increment,
  Decrement,
}

fn update(counter: &mut Counter, message: Message) {
  match message {
    Message::Increment => {
      counter.value += 1;
    }
    Message::Decrement => {
      counter.value -= 1;
    }
  }
}

fn view(counter: &Counter) -> Column<Message> {
  column![
    button("+").on_press(Message::Increment),
    text(counter.value),
    button("-").on_press(Message::Decrement)
  ]
}

fn main() {
  iced::run("Sample App", update, view).unwrap();
}
