use actix_web;
use wdms_rust2::version::SemVer;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _s: SemVer = "1.2".parse().unwrap();
    return Ok(());
}