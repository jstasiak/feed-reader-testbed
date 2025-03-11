# feed-reader-testbed

This is a server that allows verifying the behavior of Atom feed readers.

Why is this a problem? As an example some readers don't send the right HTTP
headers and, as a consequence, always fetch the whole feed which is wasteful.

See [Rachel Kroll's post on this subject](https://rachelbythebay.com/w/2023/01/18/http/).

This tool is inspired by [Rachel's feed reader score service](https://rachelbythebay.com/w/2024/05/30/fs/).

See [my blog post introducing the problem and the tool](https://stasiak.at/20250303-testing-feed-readers-with-feed-reader-testbed.html).

Quick start:

1. Run it

```
cargo run
```

2. Add http://127.0.0.1:3000 to your feed reader, refresh the feed once or twice
3. Inspect the logs

Example output (initial subscription and a refresh):

```
2025-03-11T17:43:41.531321Z  INFO serve_feed{request_id="4d4ce16e0fe4f029"}: feed_reader_testbed: User agent: feedparser/6.0.11 +https://github.com/kurtmckee/feedparser/
2025-03-11T17:43:41.531349Z  INFO serve_feed{request_id="4d4ce16e0fe4f029"}: feed_reader_testbed: If-None-Match header not provided
2025-03-11T17:43:41.531358Z  INFO serve_feed{request_id="4d4ce16e0fe4f029"}: feed_reader_testbed: If-Modified-Since header not present
2025-03-11T17:43:41.531369Z  INFO serve_feed{request_id="4d4ce16e0fe4f029"}: feed_reader_testbed: Returning 200 OK with full content
2025-03-11T17:43:51.553979Z  INFO serve_feed{request_id="2b1c60904032d9c7"}: feed_reader_testbed: User agent: feedparser/6.0.11 +https://github.com/kurtmckee/feedparser/
2025-03-11T17:43:51.554065Z  INFO serve_feed{request_id="2b1c60904032d9c7"}: feed_reader_testbed: If-None-Match header matched
2025-03-11T17:43:51.554084Z  INFO serve_feed{request_id="2b1c60904032d9c7"}: feed_reader_testbed: If-Modified-Since header matched
2025-03-11T17:43:51.554102Z  INFO serve_feed{request_id="2b1c60904032d9c7"}: feed_reader_testbed: Returning 304 Not Modified (ETag and Last-Modified match)
```

Full usage:

```
> cargo run --quiet -- --help
Usage: feed-reader-testbed [OPTIONS]

Options:
  -b, --bind <BIND>  Host address to bind to (e.g., 127.0.0.1 or 0.0.0.0) [default: 127.0.0.1]
  -p, --port <PORT>  Port number to listen on [default: 3000]
  -h, --help         Print help
  -V, --version      Print version
```
