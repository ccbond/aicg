#[macro_use]
extern crate rocket;
use rocket::form::Form;
use rocket::http::ContentType;
use rocket::response::content::Plain;
use rocket::response::Responder;
use sha1::{Digest, Sha1};
use std::env;

#[derive(FromForm)]
struct WechatVarify<'r> {
    signature: &'r str,
    timestamp: &'r str,
    nonce: &'r str,
    echostr: &'r str,
}

#[get("/", data = "<wechat_varify>")]
fn wechat_varify(wechat_varify: Form<WechatVarify<'_>>) -> u32 {
    let args: Vec<String> = env::args().collect();

    let signature = wechat_varify.signature;
    let token_key = "token";
    let token = env::var("TOKEN")
        .expect("Failed to read TOKEN from environment variable")
        .as_str();
    let timestamp = wechat_varify.timestamp;
    let nonce = wechat_varify.nonce;
    let mut tmp_array = vec![token, timestamp, nonce];
    tmp_array.sort();
    let tmp_str = tmp_array.join("");

    let mut hasher = Sha1::new();
    hasher.update(tmp_str.as_bytes());

    let hash_code = hasher.finalize();
    let hash_encode = hex::encode(hash_code);
    let hash_str = String::from_utf8(hash_encode.into()).unwrap().as_str();

    if hash_str == signature {
        let echostr = wechat_varify.echostr.parse::<u32>().unwrap();
        HttpResponse::Ok()
            .header(ContentType::Plain)
            .sized_body(Cursor::new(format!("{}", echostr)))
            .finalize()
    } else {
        HttpResponse::NotFound()
            .header(ContentType::Plain)
            .sized_body(Cursor::new("Verification failed"))
            .finalize()
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![wechat_varify])
}
