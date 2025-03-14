use axum::{
    Router,
    body::Body,
    http::{HeaderValue, StatusCode, header},
    response::Response,
    routing::get,
};
use clap::Parser;
use getrandom;
use std::net::SocketAddr;
use tracing::{Level, info, span, warn};

const X_REQUEST_ID: header::HeaderName = header::HeaderName::from_static("x-request-id");
const CACHE_CONTROL_VALUE: &str = "public, max-age=3600";

fn generate_request_id() -> String {
    let mut bytes = [0u8; 8];
    getrandom::getrandom(&mut bytes).expect("Failed to get random bytes");
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

#[derive(Parser)]
#[command(version, about)]
struct Args {
    /// Host address to bind to (e.g., 127.0.0.1 or 0.0.0.0)
    #[arg(short, long, default_value = "127.0.0.1")]
    bind: String,

    /// Port number to listen on
    #[arg(short, long, default_value_t = 3000)]
    port: u16,
}

async fn serve_index() -> Response<Body> {
    const HTML_CONTENT: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Feed Reader Testbed</title>
    <link rel="alternate" type="application/atom+xml" title="Atom Feed" href="/feed.atom">
</head>
<body>
    <h1>Feed Reader Testbed</h1>
    <p>Welcome to the Feed Reader Testbed.</p>
    <p><a href="/feed.atom">The Atom Feed</a></p>
</body>
</html>"#;

    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "text/html")
        .body(Body::from(HTML_CONTENT.to_string()))
        .unwrap()
}

async fn serve_feed(request: axum::extract::Request) -> Response<Body> {
    let request_id = generate_request_id();
    let _span = span!(Level::INFO, "serve_feed", request_id).entered();

    if let Some(referer) = request.headers().get(header::REFERER) {
        warn!("Got unexpected referer: {:?}", referer);
    }
    if let Some(cookies) = request.headers().get(header::COOKIE) {
        warn!("Got unexpected cookies: {:?}", cookies);
    }

    // Get user agent for logging
    let user_agent = request
        .headers()
        .get(header::USER_AGENT)
        .and_then(|h| h.to_str().ok())
        .unwrap_or("Not provided");
    info!("User agent: {}", user_agent);

    const FEED_CONTENT: &str = r#"<?xml version="1.0" encoding="utf-8"?>
<feed xmlns="http://www.w3.org/2005/Atom">
    <title>Testbed Feed</title>
    <link href="http://example.org/"/>
    <updated>2024-03-03T00:00:00Z</updated>
    <author>
        <name>John Doe</name>
    </author>
    <id>urn:uuid:60a76c80-d399-11d9-b93C-0003939e0af6</id>
    <entry>
        <title>Sample Entry</title>
        <link href="http://example.org/2024/03/03/sample"/>
        <id>urn:uuid:1225c695-cfb8-4ebb-aaaa-80da344efa6a</id>
        <updated>2024-03-03T00:00:00Z</updated>
        <summary>This is a sample entry in our Atom feed.</summary>
    </entry>
</feed>"#;
    let etag = HeaderValue::from_str("gei0eef3eeth1Oox9aile7ienguhaizo").unwrap();
    let last_modified = HeaderValue::from_static("Sun, 03 Mar 2024 05:00:00 GMT");

    let if_none_match = request.headers().get(header::IF_NONE_MATCH);
    let if_modified_since = request.headers().get(header::IF_MODIFIED_SINCE);

    let etag_matches = match if_none_match {
        Some(h) => {
            if h != etag {
                warn!(
                    "If-None-Match header mismatch: given {:?}, expected {:?}",
                    h, etag
                );
                false
            } else {
                info!("If-None-Match header matched");
                true
            }
        }
        None => {
            info!("If-None-Match header not provided");
            false
        }
    };
    let last_modified_matches = match if_modified_since {
        Some(h) => {
            if h != last_modified {
                warn!(
                    "If-Modified-Since header mismatch: given {:?}, expected {:?}",
                    h, last_modified
                );
                false
            } else {
                info!("If-Modified-Since header matched");
                true
            }
        }
        None => {
            info!("If-Modified-Since header not present");
            false
        }
    };

    let (status, body, message) = if etag_matches && last_modified_matches {
        (
            StatusCode::NOT_MODIFIED,
            Body::empty(),
            "Returning 304 Not Modified (ETag and Last-Modified match)",
        )
    } else if etag_matches {
        (
            StatusCode::NOT_MODIFIED,
            Body::empty(),
            "Returning 304 Not Modified (ETag match)",
        )
    } else if last_modified_matches {
        (
            StatusCode::NOT_MODIFIED,
            Body::empty(),
            "Returning 304 Not Modified (Last-Modified match)",
        )
    } else {
        (
            StatusCode::OK,
            Body::from(FEED_CONTENT.to_string()),
            "Returning 200 OK with full content",
        )
    };

    info!("{}", message);

    // Build the response with all headers
    let mut builder = Response::builder()
        .status(status)
        .header(header::ETAG, &etag)
        .header(header::LAST_MODIFIED, &last_modified)
        .header(header::CACHE_CONTROL, CACHE_CONTROL_VALUE)
        .header(X_REQUEST_ID, &request_id);

    // Add content-type only for 200 responses
    if status == StatusCode::OK {
        builder = builder.header(header::CONTENT_TYPE, "application/atom+xml");
    }

    let response = builder.body(body).unwrap();

    response
}

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let args = Args::parse();
    let addr = format!("{}:{}", args.bind, args.port)
        .parse::<SocketAddr>()
        .unwrap_or_else(|_| panic!("Failed to parse address: {}:{}", args.bind, args.port));

    let app = Router::new()
        .route("/", get(serve_index))
        .route("/feed.atom", get(serve_feed));

    info!("Server running on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap_or_else(|_| panic!("Failed to bind to address: {}", addr));
    axum::serve(listener, app).await.unwrap();
}
