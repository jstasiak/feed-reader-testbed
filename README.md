# feed-reader-testbed

This is a server that allows verifying the behavior of Atom feed readers.

Why is this a problem? As an example some readers don't send the right HTTP
headers and, as a consequence, always fetch the whole feed which is wasteful.

See [Rachel Kroll's post on this subject](https://rachelbythebay.com/w/2023/01/18/http/).

This tool is inspired by [Rachel's feed reader score service](https://rachelbythebay.com/w/2024/05/30/fs/).

Quick start:

1. Run it

```
cargo run
```

2. Add http://127.0.0.1:3000 to your feed reader, refresh the feed once or twice
3. Inspect the logs

Example output (initial subscription and a refresh):

```
2025-03-03T23:35:29.185126Z  INFO feed_reader_testbed: Returning 200 OK with full content request_id=53b035e40bf7f91e user_agent=Vienna/8414 (Macintosh; Intel macOS 15_3_0) if_none_match=None if_modified_since=None
2025-03-03T23:35:54.032359Z  INFO feed_reader_testbed: Returning 304 Not Modified (ETag and Last-Modified match) request_id=8eeb56fb8ea3c62a user_agent=Vienna/8414 (Macintosh; Intel macOS 15_3_0) if_none_match=Some("\"be363b466230b823db7c8f2f6626dc90\"") if_modified_since=Some("Sun, 03 Mar 2024 00:00:00 GMT")
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
