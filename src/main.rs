use iced::{
  widget::{button, column, row, text, text_input},
  window::settings::{PlatformSpecific, Settings},
  Element, Size, Task,
};
// mod parser;
mod parser;
mod search;

#[derive(Default)]
struct State {
  url_str: String,
  html_str: String,
}

#[derive(Debug, Clone)]
enum Message {
  ContentChanged(String),
  Search,
  SearchCompleted(String),
}

fn update(state: &mut State, message: Message) -> Task<Message> {
  match message {
    Message::ContentChanged(url_str) => {
      state.url_str = url_str;
      return Task::none();
    }
    Message::Search => {
      let url_str = state.url_str.clone();
      return Task::perform(
        async move { search::search(&url_str).unwrap() },
        Message::SearchCompleted,
      );
    }
    Message::SearchCompleted(html_str) => {
      state.html_str = html_str;
      parser::parser(&state.html_str);
      return Task::none();
    }
  }
}

fn view(state: &State) -> Element<Message> {
  let text_input = text_input("Enter URL", &state.url_str).on_input(Message::ContentChanged);
  let search_button = button("search").on_press(Message::Search);
  let search_bar = row![text_input, search_button];

  return column![search_bar.padding(45), text(&state.html_str)].into();
}

fn main() -> iced::Result {
  return iced::application("Sample App", update, view)
    .window(Settings {
      size: Size {
        width: 640.0,
        height: 480.0,
      },
      platform_specific: PlatformSpecific {
        titlebar_transparent: true,
        title_hidden: true,
        fullsize_content_view: true,
      },
      ..Default::default()
    })
    .run();
}
