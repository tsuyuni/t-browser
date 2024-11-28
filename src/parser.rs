use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct Node {
  node_type: NodeType,
  children: Vec<Node>,
}

#[derive(Debug, PartialEq)]
enum NodeType {
  Text(Text),
  Element(Element),
}

#[derive(Debug, PartialEq)]
struct Text {
  value: String,
}

#[derive(Debug, PartialEq)]
struct Element {
  tag_name: String,
  attributes: HashMap<String, String>,
}

pub fn parser(html_str: &str) -> Node {
  let tokens = tokenizer(html_str);
  let mut stack: Vec<Node> = Vec::new();
  let mut root = Node {
    node_type: NodeType::Element(Element {
      tag_name: "root".to_string(),
      attributes: HashMap::new(),
    }),
    children: Vec::new(),
  };

  for token in tokens {
    if token.starts_with('<') && token.ends_with('>') {
      if token.starts_with("</") {
        if let Some(completed_node) = stack.pop() {
          if let Some(parent) = stack.last_mut() {
            parent.children.push(completed_node);
          } else {
            root.children.push(completed_node);
          }
        }
      } else {
        let tag_content = token.trim_matches(|c| c == '<' || c == '>');
        let parts: Vec<&str> = tag_content.split_whitespace().collect();
        let tag_name = parts[0];

        let mut attributes = HashMap::new();
        for attr in &parts[1..] {
          if let Some(eq_idx) = attr.find('=') {
            let key = attr[..eq_idx].to_string();
            let value = attr[eq_idx + 1..].trim_matches('"').to_string();
            attributes.insert(key, value);
          }
        }

        let new_node = Node {
          node_type: NodeType::Element(Element {
            tag_name: tag_name.to_string(),
            attributes,
          }),
          children: Vec::new(),
        };

        stack.push(new_node);
      }
    } else {
      if let Some(last_node) = stack.last_mut() {
        last_node.children.push(Node {
          node_type: NodeType::Text(Text {
            value: token.trim().to_string(),
          }),
          children: Vec::new(),
        });
      }
    }
  }

  return root;
}

fn tokenizer(html_str: &str) -> Vec<String> {
  let mut token = String::new();
  let mut tokens = Vec::new();

  for c in html_str.chars() {
    match c {
      '<' => {
        if !token.trim().is_empty() {
          tokens.push(token.trim().to_string());
        }
        token.clear();
        token.push(c);
      }
      '>' => {
        token.push(c);
        tokens.push(token.clone());
        token.clear();
      }
      _ => {
        token.push(c);
      }
    }
  }

  return tokens;
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_tokenizer() {
    let html_str = "<h1>Example Domain</h1>";
    let tokens = tokenizer(html_str);
    assert_eq!(tokens, vec!["<h1>", "Example Domain", "</h1>"]);

    let html_str = "<a href=\"https://www.iana.org/domains/example\">More information...</a>";
    let tokens = tokenizer(html_str);
    assert_eq!(
      tokens,
      vec![
        "<a href=\"https://www.iana.org/domains/example\">",
        "More information...",
        "</a>",
      ]
    );

    let html_str =
      "<p><a href=\"https://www.iana.org/domains/example\">More information...</a></p>";
    let tokens = tokenizer(html_str);
    assert_eq!(
      tokens,
      vec![
        "<p>",
        "<a href=\"https://www.iana.org/domains/example\">",
        "More information...",
        "</a>",
        "</p>",
      ]
    );
  }

  #[test]
  fn test_parser() {
    let html_str = "<h1>Example Domain</h1>";
    let node = parser(html_str);
    assert_eq!(
      node,
      Node {
        node_type: NodeType::Element(Element {
          tag_name: "root".to_string(),
          attributes: HashMap::new(),
        }),
        children: vec![Node {
          node_type: NodeType::Element(Element {
            tag_name: "h1".to_string(),
            attributes: HashMap::new(),
          }),
          children: vec![Node {
            node_type: NodeType::Text(Text {
              value: "Example Domain".to_string(),
            }),
            children: vec![],
          }]
        }]
      }
    );

    let html_str = "<a href=\"https://www.iana.org/domains/example\">More information...</a>";
    let node = parser(html_str);
    assert_eq!(
      node,
      Node {
        node_type: NodeType::Element(Element {
          tag_name: "root".to_string(),
          attributes: HashMap::new(),
        }),
        children: vec![Node {
          node_type: NodeType::Element(Element {
            tag_name: "a".to_string(),
            attributes: vec![(
              "href".to_string(),
              "https://www.iana.org/domains/example".to_string()
            )]
            .into_iter()
            .collect(),
          }),
          children: vec![Node {
            node_type: NodeType::Text(Text {
              value: "More information...".to_string(),
            }),
            children: vec![],
          }]
        }]
      }
    );

    let html_str =
      "<p><a href=\"https://www.iana.org/domains/example\">More information...</a></p>";
    let node = parser(html_str);
    assert_eq!(
      node,
      Node {
        node_type: NodeType::Element(Element {
          tag_name: "root".to_string(),
          attributes: HashMap::new(),
        }),
        children: vec![Node {
          node_type: NodeType::Element(Element {
            tag_name: "p".to_string(),
            attributes: HashMap::new(),
          }),
          children: vec![Node {
            node_type: NodeType::Element(Element {
              tag_name: "a".to_string(),
              attributes: vec![(
                "href".to_string(),
                "https://www.iana.org/domains/example".to_string()
              )]
              .into_iter()
              .collect(),
            }),
            children: vec![Node {
              node_type: NodeType::Text(Text {
                value: "More information...".to_string(),
              }),
              children: vec![],
            }]
          }]
        }]
      }
    );
  }
}
