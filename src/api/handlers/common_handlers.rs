use log::info;

pub async fn index() -> &'static str {
    info!("Requested.");
    "OK!"
}