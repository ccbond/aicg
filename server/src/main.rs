use std::{env, net::SocketAddr};

use axum::{
    extract::Form,
    routing::get,
    Router,
};
use serde::Deserialize;
use sha1::{Digest, Sha1};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/wechat/varified", get(wechat_verified));

    let addr = SocketAddr::from(([127,0,0,1], 80));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Deserialize, Debug)]
struct WechatVerified {
    signature: String,
    timestamp: String,
    nonce: String,
    echostr: String,
}

async fn wechat_verified(Form(wechat_verified): Form<WechatVerified>) ->String {
    let signature = wechat_verified.signature;
    let token = env::var("TOKEN").expect("Failed to read TOKEN from environment variable");
    let timestamp = wechat_verified.timestamp;
    let nonce = wechat_verified.nonce;

    let mut tem_arr = vec![token, timestamp, nonce];
    tem_arr.sort();
    let tem_str = tem_arr.join("");

    let mut hasher = Sha1::new();
    hasher.update(tem_str.as_bytes());
    let hash_code = hasher.finalize();
    let hash_encode = hex::encode(hash_code);
    let binding = String::from_utf8(hash_encode.into()).unwrap();

    if binding.eq(&signature) {
        wechat_verified.echostr
    } else {
        String::from("error")
    }
}