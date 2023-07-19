use anyhow::{Result, anyhow};
use bytes::Bytes;
use serde::{Serialize, Deserialize};
use simplehttp::simplehttp_spin::SimpleHttpClientSpin;
use spin_sdk::{
    http::{Request, Response},
    http_component,
};
#[derive(Serialize,Deserialize,Debug)]
struct Actor {
    first_name: String,
    last_name: String,
    id: String,
    films: Vec<String>,
}

/// A simple Spin HTTP component.
#[http_component]
fn handle_surrealdb_spin(req: Request) -> Result<Response> {
    let id: i32 = req.headers()
        .get("spin-path-info")
        .ok_or(anyhow::anyhow!("missing path"))?
        .to_str()?
        [1..]
        .parse()?;
    let mut surreal_client = surrealdb_http::surreal::SurrealDbClient::new("root", "root", "http://localhost:8000", "myns", "mydb", SimpleHttpClientSpin::new_spin());
    let actor: Actor = surreal_client
        .query_single(&format!("SELECT *,->played_in->film.title as films FROM actor WHERE id=actor:{}",id)).unwrap()
        .result
        .pop()
        .ok_or(anyhow!("unknown id"))?;
    
    Ok(http::Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .body(Some(Bytes::from(serde_json::to_vec(&actor)?)))?)
}
