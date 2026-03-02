use tokio::{runtime::Id, select};
use warp::{filters::path::param, reply::{Reply, Response}, Filter};
use std::{collections::HashMap, fs, os::unix::process, process::Output};
use regex::Regex;
use dotenvy::dotenv;
use urlencoding::encode;
use std::{env, process::{Command}};
use sqlorm::{prelude::*, sqlx::postgres::PgPoolOptions};

use std::net::SocketAddrV6;

mod request;
mod macros;

#[table(name="teszt")]
#[derive(Default, Clone, Debug)]
pub struct Szam {
    #[sql(pk)]
    pub id: i32,

    pub szam:i32,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let port = 4040;
    println!("running on {}", port);
    let uri = "postgresql://postgres.xqnvldpayoaloqvqxirr:!ALMASpite456!@aws-1-eu-west-1.pooler.supabase.com:6543/postgres";
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .test_before_acquire(true)
        .connect(uri).await.unwrap();

    //path
    let home = warp::path::end().map(|| warp::reply::html(fs::read_to_string("html/index.html").unwrap()));
    let style = warp::path!("style.css").and(warp::fs::file("html/style.css"));
    let script = warp::path!("script.js").and(warp::fs::file("script.js"));

    //"postgresql://postgres:[YOUR-PASSWORD]@db.your_project_id.supabase.co:5432/postgres";
    //postgresql://postgres.xqnvldpayoaloqvqxirr:[YOUR-PASSWORD]@aws-1-eu-west-1.pooler.supabase.com:6543/postgres
   // let pool = Pool::connect("postgresql://postgres.xqnvldpayoaloqvqxirr:!ALMASpite456!@aws-1-eu-west-1.pooler.supabase.com:6543/postgres").await.unwrap();


    //let num = sqlx::query("SELECT * FROM teszt").persistent(false).fetch_all(&pool).await.unwrap();
    //let innserted = sqlx::query("INSERT INTO teszt (szam) VALUES ($1) RETURNING id") .persistent(false).bind(35).fetch_one(&pool).await.unwrap();
    //println!("{:?}", num);
    //println!("{:?}", innserted);

    let routes = home.or(style).or(script);
    warp::serve(routes).run(([0,0,0,0], port)).await;
}

