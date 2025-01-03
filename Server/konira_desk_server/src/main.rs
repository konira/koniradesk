use std::{convert::Infallible, fs, io, sync::Arc};

use openssl::ssl::{AlpnError, SslAcceptor, SslFiletype, SslMethod};
use quinn::ServerConfig;
use xitca_http::{
    h1, h2, h3,
    http::{const_header_value::TEXT_UTF8, header::CONTENT_TYPE, Request, RequestExt, Response, Version},
    util::middleware::{Logger, SocketConfig},
    HttpServiceBuilder, ResponseBody,
};
use xitca_service::{fn_service, ServiceExt};
const KEY: &str = "/home/konira/RustroverProjects/konira_rdp/Shared/certs/key.pem";
const CERT: &str = "/home/konira/RustroverProjects/konira_rdp/Shared/certs/cert.pem";
fn main() -> io::Result<()> {
    tracing_subscriber::fmt().with_max_level(tracing::Level::INFO).init();

    // construct http2 openssl config.
    let acceptor = h2_config()?;

    // construct http3 quic server config
    let config = h3_config()?;

    // construct server
    xitca_server::Builder::new()
        // bind to a http/1 service.
        .bind(
            "http/1",
            "127.0.0.1:8080",
            fn_service(handler_h1)
                .enclosed(Logger::new())
                .enclosed(HttpServiceBuilder::h1())
                .enclosed(SocketConfig::new()),
        )?
        // bind to a http/2 service.
        // *. http/1 and http/2 both use tcp listener so it should be using a separate port.
        .bind(
            "http/2",
            "127.0.0.1:8081",
            fn_service(handler_h2).enclosed(HttpServiceBuilder::h2().openssl(acceptor)),
        )?
        // bind to a http/3 service.
        // *. note the service name must be unique.
        //
        // Bind to same service with different bind_xxx API is allowed for reusing one service
        // on multiple socket addresses and protocols.
        .bind_h3(
            "http/3",
            "127.0.0.1:8081",
            config,
            fn_service(handler_h3).enclosed(HttpServiceBuilder::h3()),
        )?
        .build()
        .wait()
}

async fn handler_h1(_: Request<RequestExt<h1::RequestBody>>) -> Result<Response<ResponseBody>, Infallible> {
    Ok(Response::builder()
        .header(CONTENT_TYPE, TEXT_UTF8)
        .body("Hello World from Http/1!".into())
        .unwrap())
}

async fn handler_h2(
    _: Request<RequestExt<h2::RequestBody>>,
) -> Result<Response<ResponseBody>, Box<dyn std::error::Error>> {
    let res = Response::builder()
        .status(200)
        // possible redirect browser to http/3 service.
        .header("Alt-Svc", "h3=\":8081\"; ma=86400")
        .header(CONTENT_TYPE, TEXT_UTF8)
        .body("Hello World from Http/2!".into())?;
    Ok(res)
}

async fn handler_h3(
    _: Request<RequestExt<h3::RequestBody>>,
) -> Result<Response<ResponseBody>, Box<dyn std::error::Error>> {
    Response::builder()
        .status(200)
        .version(Version::HTTP_3)
        .header(CONTENT_TYPE, TEXT_UTF8)
        .body("Hello World from Http/3!".into())
        .map_err(Into::into)
}

fn h2_config() -> io::Result<SslAcceptor> {
    // set up openssl and alpn protocol.
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls())?;
    builder.set_private_key_file(KEY, SslFiletype::PEM)?;
    builder.set_certificate_chain_file(CERT)?;

    builder.set_alpn_select_callback(|_, protocols| {
        const H2: &[u8] = b"\x02h2";
        const H11: &[u8] = b"\x08http/1.1";

        if protocols.windows(3).any(|window| window == H2) {
            Ok(b"h2")
        } else if protocols.windows(9).any(|window| window == H11) {
            Ok(b"http/1.1")
        } else {
            Err(AlpnError::NOACK)
        }
    });

    builder.set_alpn_protos(b"\x08http/1.1\x02h2")?;

    Ok(builder.build())
}

fn h3_config() -> io::Result<ServerConfig> {
    let cert = fs::read(CERT)?;
    let key = fs::read(KEY)?;

    let key = rustls_pemfile::pkcs8_private_keys(&mut &*key).next().unwrap().unwrap();
    let key = quinn::rustls::pki_types::PrivateKeyDer::from(key);

    let cert = rustls_pemfile::certs(&mut &*cert).collect::<Result<_, _>>().unwrap();

    let mut config = quinn::rustls::ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(cert, key)
        .unwrap();

    config.alpn_protocols = vec![b"h3".to_vec()];

    let config = quinn::crypto::rustls::QuicServerConfig::try_from(config).unwrap();

    Ok(ServerConfig::with_crypto(Arc::new(config)))
}