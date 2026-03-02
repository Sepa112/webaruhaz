use tokio::{runtime::Id, select};
use warp::{filters::path::param, reply::{Reply, Response}, Filter};
use std::{collections::HashMap, fs, os::unix::process, process::Output};
use regex::Regex;
use dotenvy::dotenv;
use urlencoding::encode;
use std::{env, process::{Command}};

mod request;
mod macros;
use request::RequestBuilder;


#[tokio::main]
async fn main() {
    dotenv().ok();
    let port = 4040;
    println!("running on {}", port);

    //path
    let home = warp::path::end().map(|| warp::reply::html(fs::read_to_string("html/index.html").unwrap()));
    let style = warp::path!("style.css").and(warp::fs::file("html/style.css"));
    let script = warp::path!("script.js").and(warp::fs::file("script.js"));

    let routes = home.or(style).or(script);
    warp::serve(routes).run(([0,0,0,0], port)).await;
}

