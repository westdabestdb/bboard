use actix::{Actor, StreamHandler};
use actix_web::{web, HttpResponse, HttpRequest, Error};
use actix_web_actors::ws;

struct MyWs;

impl Actor for MyWs {
  type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
  fn handle(
      &mut self,
      msg: Result<ws::Message, ws::ProtocolError>,
      ctx: &mut Self::Context,
  ) {
      println!("WS: {:?}", msg);
      match msg {
          Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
          Ok(ws::Message::Text(text)) => ctx.text(text),
          Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
          _ => (),
      }
  }
}

async fn ws_handler(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
  let resp = ws::start(MyWs {}, &req, stream);
  println!("{:?}", resp);
  resp
}

async fn health() -> HttpResponse {
  HttpResponse::Ok().body("server is up!")
}

pub fn app_config(config: &mut web::ServiceConfig) {
  config
    .service(web::resource("/").route(web::get().to(health)))
    .route("/ws/", web::get().to(ws_handler));
}