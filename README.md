# abbreviator

URL Shortener for the `swissgeol` subsurface viewer.

## quickstart

Build the docker image:

```bash
docker build -t abbreviator:test -f Dockerfile .
```

Run it:

```bash
docker run -d -p 8080:8080 \
    -e DATABASE_URL='sqlite::memory:' \
    -e ID_LENGTH='5' \
    -e PORT='8080' \
    abbreviator:test
```

This exposes the service on `localhost:8080` with an in-memory sqlite database.


Sample request to shorten a url:

```bash
curl -v localhost:8080/ -d '{ "url": "https://github.com/swissgeol/abbreviator.git" }'
```

The `Location` header contains the shortened url to retrieve the original url:

```bash
# replace id with the last path segment of the shortened url
curl -v localhost:8080/{id} 
```

This returns a response with status `301` and the original url in the `Location` header.

## develop

[Install Rust](https://www.rust-lang.org/tools/install) then:

```bash
cargo build
cargo test
cargo run
```
