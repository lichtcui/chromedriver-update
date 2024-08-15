use chromedriver_update::ChromeDriver;

#[tokio::main]
async fn main() {
    let mut driver = ChromeDriver::new();
    driver.init().await.unwrap();

    println!("driver version {}", driver.version);
    println!("browser version {}", driver.browser_version);

    match driver.try_download().await {
        Ok(_) => println!("success download driver"),
        Err(err) => eprintln!("failed download driver {}", err),
    }
}
