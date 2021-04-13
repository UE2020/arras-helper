use async_std::net::TcpStream;
use async_tungstenite::{MaybeTlsStream, WebSocketStream};
use std::collections::HashMap;
use tungstenite::handshake::client::{Request, Response};
use tungstenite::Message;
use url::Url;

use async_std::prelude::*;
use futures::SinkExt;
use futures::StreamExt;

use crate::captcha;
use crate::protocol;

async fn connect_tls_disable_cert_verification<R>(
    req: R,
) -> std::result::Result<(WebSocketStream<MaybeTlsStream<TcpStream>>, Response), tungstenite::Error>
where
    R: Into<Request<'static>> + Unpin,
{
    use async_tls::TlsConnector;
    use rustls::ClientConfig;
    use std::sync::Arc;

    mod danger {
        use webpki;

        pub struct NoCertificateVerification {}

        impl rustls::ServerCertVerifier for NoCertificateVerification {
            fn verify_server_cert(
                &self,
                _roots: &rustls::RootCertStore,
                _presented_certs: &[rustls::Certificate],
                _dns_name: webpki::DNSNameRef<'_>,
                _ocsp: &[u8],
            ) -> Result<rustls::ServerCertVerified, rustls::TLSError> {
                Ok(rustls::ServerCertVerified::assertion())
            }
        }
    }

    let mut request: Request = req.into();
    let mut config = ClientConfig::new();
    config
        .dangerous()
        .set_certificate_verifier(Arc::new(danger::NoCertificateVerification {}));
    let connector = TlsConnector::from(Arc::new(config));

    request.add_header(
        std::borrow::Cow::from("origin"),
        std::borrow::Cow::from("https://arras.io"),
    );
    async_tungstenite::connect_async_with_tls_connector(request, Some(connector)).await
}

pub struct Bot {
    pub socket: WebSocketStream<MaybeTlsStream<TcpStream>>
}

impl Bot {
    pub async fn connect(url: String, name: String) -> Self {
        let recaptcha_code = captcha::make_key();
        let ip = url;
        let url = Url::parse(ip.as_str()).unwrap();
        let mut client = connect_tls_disable_cert_verification(url).await.unwrap().0;
        client
            .send(Message::Binary(protocol::encode_outbound(
                protocol::Outbound::Ping(1),
            )))
            .await
            .unwrap();
        client
            .send(Message::Binary(protocol::encode_outbound(
                protocol::Outbound::D(1),
            )))
            .await
            .unwrap();
        client
            .send(Message::Binary(protocol::encode_outbound(
                protocol::Outbound::Type(String::from("{\"type\":\"headless\"}")),
            )))
            .await
            .unwrap();
        client
            .send(Message::Binary(protocol::encode_outbound(
                protocol::Outbound::K,
            )))
            .await
            .unwrap();
        client
            .send(Message::Binary(protocol::encode_outbound(
                protocol::Outbound::Spawn {
                    name,
                    captcha: recaptcha_code,
                },
            )))
            .await
            .unwrap();
        Self {
            socket: client
        }
    }
}
