use iced::{
  widget::{self, button, column, container::Style, row, text_input, Container, Text},
  window::settings::{PlatformSpecific, Settings},
  Background, Color, Element, Length, Size, Task,
};

// mod parser;
mod parser;
mod search;

#[derive(Default)]
struct State {
  url_str: String,
  html_str: String,
  parsed_node: parser::Node,
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
      // parser::parser(&state.html_str);
      state.parsed_node = parser::parser("
      <html>
        <body>
          <div>
            <h1>heading</h1>
            <p>text</p>
          </div>
        </body>
      </html>
      ");
      return Task::none();
    }
  }
}

fn view(state: &State) -> Element<Message> {
  let text_input = text_input("Enter URL", &state.url_str).on_input(Message::ContentChanged);
  let search_button = button("search").on_press(Message::Search);
  let search_bar = row![text_input, search_button];

  let render = Container::new(
    Container::new(
      Container::new(
        column![
          widget::Text::new("Example Domain")
            .style(|_theme| widget::text::Style {
              color: Some(Color::BLACK),
            })
            .size(32),
      
          widget::Text::new("This domain is for use in illustrative examples in documents. You may use this domain in literature without prior coordination or asking for permission.")
            .style(|_theme| widget::text::Style {
              color: Some(Color::BLACK),
            })
            .size(16),
          
          Container::new(
            widget::Text::new("More information...")
              .style(|_theme| widget::text::Style {
                color: Some(Color::from_rgb(0.0, 0.0, 1.0)),
              })
              .size(16)
          )
        ]
      )
        .width(Length::Fill)
        .height(Length::Shrink)
    )
      .width(Length::Fill)
      .height(Length::Fill)
      .padding([8, 8])
  )
    .width(Length::Fill)
    .height(Length::Fill)
    .style(|_theme| Style {
      background: Some(Background::Color(Color::WHITE)),
      ..Style::default()
    });

  return column![search_bar.padding(45), render].into();
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
