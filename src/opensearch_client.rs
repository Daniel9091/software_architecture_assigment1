use opensearch::{
    auth::Credentials,
    cert::CertificateValidation,
    http::transport::{SingleNodeConnectionPool, TransportBuilder},
    OpenSearch,
};
use url::Url;

pub fn os_client() -> OpenSearch {
    let node = std::env::var("OS_NODE").unwrap_or_else(|_| "https://opensearch:9200".into());
    let user = std::env::var("OS_USER").unwrap_or_else(|_| "admin".into());
    let pass = std::env::var("OS_PASS").unwrap_or_else(|_| "changeme".into());

    let url = Url::parse(&node).expect("OS_NODE inválido");
    let pool = SingleNodeConnectionPool::new(url);

    let transport = TransportBuilder::new(pool)
        .auth(Credentials::Basic(user, pass))
        .cert_validation(CertificateValidation::None) // DEV self-signed
        .build()
        .expect("No se pudo construir el transporte");

    OpenSearch::new(transport)
}
