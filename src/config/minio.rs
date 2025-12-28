use aws_config::Region;
use aws_sdk_s3::{Client, config::Credentials};

pub async fn new_minio_storage(
    reg: &str,
    access: &str,
    secret: &str,
    endpoint: &str,
) -> Client {
    let region = Region::new(reg.to_string());

    let cred = Credentials::new(access, secret, None, None, "minio");

    let config = aws_sdk_s3::Config::builder()
        .region(region)
        .credentials_provider(cred)
        .endpoint_url(endpoint)
        .force_path_style(true)
        .build();

    Client::from_conf(config)
}
