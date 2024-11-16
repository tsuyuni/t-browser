use http_body_util::BodyExt;
use http_body_util::Empty;
use hyper::{body::Bytes, Request};
use hyper_util::rt::TokioIo;
use tokio::net::TcpStream;

#[tokio::main]
pub async fn search(url_str: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
  let url = url_str.parse::<hyper::Uri>()?;
  let host = url.host().expect("uri has no host");
  let port = url.port_u16().unwrap_or(80);
  let address = format!("{}:{}", host, port);
  let stream = TcpStream::connect(address).await?;
  let io = TokioIo::new(stream);
  let (mut sender, conn) = hyper::client::conn::http1::handshake(io).await?;
  tokio::task::spawn(async move {
    if let Err(err) = conn.await {
      println!("Connection failed: {:?}", err);
    }
  });

  let authority = url.authority().unwrap().clone();
  let req = Request::builder()
    .uri(url)
    .header(hyper::header::HOST, authority.as_str())
    .body(Empty::<Bytes>::new())?;
  let mut res = sender.send_request(req).await?;
  println!("Response status: {}", res.status());

  let mut body = String::new();
  while let Some(next) = res.frame().await {
    let frame = next?;
    if let Some(chunk) = frame.data_ref() {
      body.push_str(&String::from_utf8_lossy(chunk));
    }
  }

  return Ok(body);
}
