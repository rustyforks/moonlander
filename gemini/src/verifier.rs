use rustls::{ServerCertVerified, ServerCertVerifier, TLSError};

pub struct GeminiVerifier {}

impl GeminiVerifier {
    pub fn new() -> Self {
        Self {}
    }
}

// big TODO: TOFU

impl ServerCertVerifier for GeminiVerifier {
    fn verify_server_cert(
        &self,
        _roots: &rustls::RootCertStore,
        _presented_certs: &[rustls::Certificate],
        _dns_name: webpki::DNSNameRef,
        _ocsp_response: &[u8],
    ) -> Result<ServerCertVerified, TLSError> {
        Ok(ServerCertVerified::assertion())
    }
}
