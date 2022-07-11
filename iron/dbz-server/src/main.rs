extern crate iron;
extern crate iron_test;
use iron::prelude::*;
use iron::status;
use iron::Handler;
use iron::Timeouts;
#[allow(unused_imports)]
use iron_test::{request, response};
use router::Router;
use std::io::Read;
use std::time::Duration;
fn main() {
    let mut router = Router::new();
    router
        .get("/", eu_sou_goku, "index")
        .post("/", retrucar, "post")
        .get("/:query", ola_vc, "query");

    Iron {
        handler: router,
        threads: 8,
        timeouts: Timeouts {
            keep_alive: Some(Duration::from_secs(10)),
            read: Some(Duration::from_secs(10)),
            write: Some(Duration::from_secs(10)),
        },
    }
    .http("localhost:3000")
    .unwrap();
}

fn eu_sou_goku(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Oi, eu sou Goku!")))
}

fn retrucar(request: &mut Request) -> IronResult<Response> {
    let mut body = Vec::new();
    request
        .body
        .read_to_end(&mut body)
        .map_err(|e| IronError::new(e, (status::InternalServerError, "Error ao ler o request")))?;
    Ok(Response::with((status::Ok, body)))
}

fn ola_vc(request: &mut Request) -> IronResult<Response> {
    let ref query = request
        .extensions
        .get::<Router>()
        .unwrap()
        .find("query")
        .unwrap_or("/");
    Ok(Response::with((status::Ok, textinho(query))))
}

fn textinho(query: &str) -> String {
    let mut ola_string = "Olá".to_string();
    let eu_sou = r"\eu sou o Goku";
    ola_string.push_str(query);
    ola_string.push_str(eu_sou);
    ola_string
}

// Aqui para baixo é parte do teste
struct GokuHandler;
impl Handler for GokuHandler {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "Oi, eu sou o Goku")))
    }
}
#[cfg(test)]
mod tests {
    use iron::Headers;

    use super::*;
    #[test]
    fn test_goku_handler() {
        let response =
            request::get("http://localhost:3000/", Headers::new(), &GokuHandler).unwrap();
    }
}
