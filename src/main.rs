use xplorer::service::Service;

mod xplorer;

#[tokio::main]
async fn main() {
    env_logger::init();
    let xplorer = xplorer::xplorer::Xplorer::start(Service::new(
        "testing".to_string(),
        "192.168.1.1".to_string(),
        8081,
        xplorer::service::ServiceType::SERVICE_TYPE_UDP,
    ));

    xplorer.stop().await
}
