use std::net::SocketAddr;
use std::{error::Error, future::Future};

use bytes::Bytes;
use http::{header, Method, Request, StatusCode, Version};
use http_body_util::{BodyExt, Empty};
use hyper_util::rt::TokioIo;
use tokio::io::{split, AsyncRead, AsyncWrite};
use tokio::net::windows::named_pipe::ClientOptions;
use tokio::net::windows::named_pipe::NamedPipeClient;

#[derive(Clone)]
struct TokioExecutor;

impl<F> hyper::rt::Executor<F> for TokioExecutor
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    fn execute(&self, future: F) {
        tokio::spawn(future);
    }
}

static ALPN: &[u8] = b"h3";

#[derive(structopt::StructOpt, Debug)]
#[structopt(name = "server")]
struct Opt {
    #[structopt(
        long,
        short,
        default_value = "examples/ca.cert",
        help = "Certificate of CA who issues the server certificate"
    )]
    pub ca: std::path::PathBuf,

    #[structopt(name = "keylogfile", long)]
    pub key_log_file: bool,

    #[structopt()]
    pub uri: String,
}

async fn http2_hyper() -> Result<(), Box<dyn Error>> {
    const PIPE_NAME: &str = r"\\.\pipe\koniradesk-01";

    let pipe = ClientOptions::new().open(PIPE_NAME)?;
    let io = TokioIo::new(pipe);

    let (mut request_sender, connection) =
        hyper::client::conn::http2::handshake(TokioExecutor, io).await?;

    // spawn a task to poll the connection and drive the HTTP state
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Error in connection: {}", e);
        }
    });

    let request = Request::builder()
        // We need to manually add the host header because SendRequest does not
        .uri("http://localhost:5179/ping")
        .method("GET")
        .version(Version::HTTP_2)
        .body(Empty::<Bytes>::new())
        .unwrap();

    let response = request_sender.send_request(request).await?;
    // response.into_body().collect().await?;

    let _body = response.into_body();
    let _body = _body.collect().await?.to_bytes();

    print!("Response: {:?}", _body);

    Ok(())
}
async fn http2() -> Result<(), Box<dyn Error>> {
    const PIPE_NAME: &str = r"\\.\pipe\koniradesk-01";

    let pipe = ClientOptions::new().open(PIPE_NAME)?;

    let (h2, connection) = h2::client::handshake(pipe).await?;
    tokio::spawn(async move {
        connection.await.unwrap();
    });
    let mut h2 = h2.ready().await?;
    // Prepare the HTTP request to send to the server.
    let request = Request::builder()
        .method(Method::GET)
        .uri("http://localhost:5179/ping")
        .body(())
        .unwrap();

    // Send the request. The second tuple item allows the caller
    // to stream a request body.
    let (response, _) = h2.send_request(request, false).unwrap();

    let (head, mut body) = response.await?.into_parts();

    println!("Received response: {:?}", head);

    let _id = body.stream_id();
    let mut flow_control = body.flow_control().clone();

    while let Some(chunk) = body.data().await {
        let chunk = chunk?;
        println!("RX: {:?}", chunk);

        // Let the server send more data.
        let _ = flow_control.release_capacity(chunk.len());
    }

    Ok(())
}
async fn http3() -> Result<(), Box<dyn Error>> {
    use futures::future;
    use std::{path::PathBuf, sync::Arc};
    use tokio::io::AsyncWriteExt;
    use tracing::{error, info};

    let opt = Opt {
        ca: PathBuf::from("examples/ca.cert"),
        key_log_file: false,
        uri: "https://localhost:4443/ping".to_string(),
    };
    // DNS lookup

    let uri = opt.uri.parse::<http::Uri>()?;

    if uri.scheme() != Some(&http::uri::Scheme::HTTPS) {
        Err("uri scheme must be 'https'")?;
    }

    let auth = uri.authority().ok_or("uri must have a host")?.clone();

    let port = auth.port_u16().unwrap_or(443);

    let addr = tokio::net::lookup_host((auth.host(), port))
        .await?
        .next()
        .ok_or("dns found no addresses")?;

    info!("DNS lookup for {:?}: {:?}", uri, addr);

    // create quinn client endpoint

    // load CA certificates stored in the system
    let mut roots = rustls::RootCertStore::empty();
    match rustls_native_certs::load_native_certs() {
        Ok(certs) => {
            for cert in certs {
                if let Err(e) = roots.add(&rustls::Certificate(cert.0)) {
                    error!("failed to parse trust anchor: {}", e);
                }
            }
        }
        Err(e) => {
            error!("couldn't load any default trust roots: {}", e);
        }
    };

    // load certificate of CA who issues the server certificate
    // NOTE that this should be used for dev only
    // if let Err(e) = roots.add(&rustls::Certificate(std::fs::read(opt.ca)?)) {
    //     error!("failed to parse trust anchor: {}", e);
    // }

    let mut tls_config = rustls::ClientConfig::builder()
        .with_safe_default_cipher_suites()
        .with_safe_default_kx_groups()
        .with_protocol_versions(&[&rustls::version::TLS13])?
        .with_root_certificates(roots)
        .with_no_client_auth();

    tls_config.enable_early_data = true;
    tls_config.alpn_protocols = vec![ALPN.into()];

    // optional debugging support
    if opt.key_log_file {
        // Write all Keys to a file if SSLKEYLOGFILE is set
        // WARNING, we enable this for the example, you should think carefully about enabling in your own code
        tls_config.key_log = Arc::new(rustls::KeyLogFile::new());
    }

    let mut client_endpoint = h3_quinn::quinn::Endpoint::client("[::]:0".parse().unwrap())?;

    let client_config = quinn::ClientConfig::new(Arc::new(tls_config));
    client_endpoint.set_default_client_config(client_config);

    let conn = client_endpoint.connect(addr, auth.host())?.await?;

    info!("QUIC connection established");

    // create h3 client

    // h3 is designed to work with different QUIC implementations via
    // a generic interface, that is, the [`quic::Connection`] trait.
    // h3_quinn implements the trait w/ quinn to make it work with h3.
    let quinn_conn = h3_quinn::Connection::new(conn);

    let (mut driver, mut send_request) = h3::client::new(quinn_conn).await?;

    let drive = async move {
        future::poll_fn(|cx| driver.poll_close(cx)).await?;
        Ok::<(), Box<dyn std::error::Error>>(())
    };

    // In the following block, we want to take ownership of `send_request`:
    // the connection will be closed only when all `SendRequest`s instances
    // are dropped.
    //
    //             So we "move" it.
    //                  vvvv
    let request = async move {
        info!("sending request ...");

        let req = http::Request::builder()
        .header("Content-Type", "application/octet-stream")
        .header("Content-Encoding", "gzip")
        .header("Content-Codec", "h")
        .header("cod", "12133")
        .uri(uri).body(())?;

        // sending request results in a bidirectional stream,
        // which is also used for receiving response
        let mut stream = send_request.send_request(req).await?;

        // finish on the sending side
        stream.finish().await?;

        info!("receiving response ...");

        let resp = stream.recv_response().await?;

        info!("response: {:?} {}", resp.version(), resp.status());
        info!("headers: {:#?}", resp.headers());

        // `recv_data()` must be called after `recv_response()` for
        // receiving potential response body
        while let Some(mut chunk) = stream.recv_data().await? {
            let mut out = tokio::io::stdout();
            out.write_all_buf(&mut chunk).await?;
            out.flush().await?;
        }
        //stream.send_data(Bytes::new()).await?;

        Ok::<_, Box<dyn std::error::Error>>(())
    };

    let (req_res, drive_res) = tokio::join!(request, drive);
    req_res?;
    drive_res?;

    // wait for the connection to be closed before exiting
    client_endpoint.wait_idle().await;

    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::modules::tracing_setup;

    pub fn setup() -> () {
        tracing_setup::init_tracing().unwrap();
        ()
    }

    #[tokio::test]
    async fn test_http2() {
        setup();
        let _ = http2().await;
    }

    #[tokio::test]
    async fn test_http3() {
        setup();
        let _ = http3().await;
    }
}
