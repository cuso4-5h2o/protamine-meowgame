use actix_files as fs;
use actix_ratelimit::{MemoryStore, MemoryStoreActor, RateLimiter};
use actix_web::{post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use crypto::digest::Digest;
use crypto::sha1::Sha1;
use log::info;
use regex::Regex;
use serde::Deserialize;
use simplelog::*;
use std::fs::OpenOptions;
use std::time::Duration;

fn launch_info(bind: String) {
    println!("Protamine Server for Meowgame");
    println!("Listening on http://{}", bind);
}

fn verify_game(score: u32, picking_log: String) -> bool {
    let picking_log: Vec<&str> = picking_log.split(",").collect();
    let mut max_score: u32 = 0;
    for this_score_str in picking_log.iter() {
        let this_score = this_score_str.parse::<i64>().unwrap();
        if this_score > 1350 {
            return false;
        }
        max_score += this_score as u32;
    }
    return (max_score >= score) && (max_score <= (score + 2000000000));
}

fn generate_flag(id: String) -> String {
    let mut output = [0; 40];
    let mut hasher = Sha1::new();
    let salt = "hyTreKUPqqMh9JMq!NsuPzQHJYk@4kALDC5S*#jUPEVN8BAhdaDQVBvUm5@o#!t%";
    hasher.input(id.as_bytes());
    hasher.input(salt.as_bytes());
    hasher.result(&mut output);
    let mut sha1_result = "".to_string();
    for index in (10..15).rev() {
        sha1_result.push_str(&format!("{:02x}", output[index]));
    }
    hasher.reset();
    return format!("miao{{p_{}}}", &sha1_result.to_string());
}

#[derive(Deserialize)]
struct GameInfo {
    id: String,
    score: String,
    picking_log: String,
}

#[post("/api/report")]
async fn report_game(_req: HttpRequest, game_info: web::Json<GameInfo>) -> impl Responder {
    let id = Regex::new(r"[^0-9A-z+/=]")
        .unwrap()
        .replace_all(game_info.id.as_str(), "")
        .to_string();
    let score = Regex::new(r"[^0-9]")
        .unwrap()
        .replace_all(game_info.score.as_str(), "")
        .parse::<u32>()
        .unwrap();
    let picking_log = Regex::new(r"[^0-9,\-]")
        .unwrap()
        .replace_all(game_info.picking_log.as_str(), "")
        .to_string();
    let score_goal = 50000000;
    let result: String;
    let ip = _req
        .headers()
        .get("CF-Connecting-IP")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    if score >= score_goal {
        if verify_game(score, picking_log.to_string()) {
            info!(
                "{} got flag, IP is {}, picking_log is {}",
                id, ip, picking_log
            );
            result = generate_flag(id);
        } else {
            info!(
                "{} cheating was detected, IP is {}, picking_log is {}",
                id, ip, picking_log
            );
            result = "Don't cheat.".to_string();
        }
    } else {
        info!(
            "{} sent too low score, IP is {}, picking_log is {}",
            id, ip, picking_log
        );
        result = "Score is too low.".to_string();
    }
    HttpResponse::Ok().body(result)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let bind = "0.0.0.0:8080";
    let store = MemoryStore::new();
    let log_file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("./log/server.log")
        .unwrap();
    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Warn, Config::default(), TerminalMode::Mixed),
        WriteLogger::new(LevelFilter::Info, Config::default(), log_file),
    ])
    .unwrap();
    launch_info(bind.to_string());
    HttpServer::new(move || {
        App::new()
            .data(web::JsonConfig::default().limit(20 * 1024))
            .wrap(
                RateLimiter::new(MemoryStoreActor::from(store.clone()).start())
                    .with_interval(Duration::from_secs(600))
                    .with_max_requests(125)
                    .with_identifier(|req| {
                        Ok(req
                            .headers()
                            .get("CF-Connecting-IP")
                            .unwrap()
                            .to_str()
                            .unwrap()
                            .to_string())
                    }),
            )
            .service(report_game)
            .service(fs::Files::new("/", "./web-mobile").index_file("index.html"))
    })
    .bind(bind)?
    .run()
    .await
}
