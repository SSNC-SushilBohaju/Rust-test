mod aws_connection;
use aws_connection::bootstrap;
use constant::ISSUE_AND_ASSOCIATE_DEVICE_ID_URL;

#[tokio::main]
async fn main() {
    let url:&str =ISSUE_AND_ASSOCIATE_DEVICE_ID_URL;
    bootstrap::auth_download_file(url).await;
    println!("auth download success");

}

