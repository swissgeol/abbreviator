# abbreviator

URL Shortener for the `swissgeol` subsurface viewer.

## Quickstart

Build and run with docker:

```bash
# build
docker build -t abbreviator:test -f Dockerfile .
# run
docker run -d -p 8080:8080 \
    -e DATABASE_URL='sqlite::memory:' \
    -e HOST_WHITELIST='beta.swissgeol.ch' \
    abbreviator:test
```

This exposes the service on `localhost:8080` with an in-memory sqlite database.


Sample request to shorten a url:

```bash
curl -v localhost:8080/ -d '{ "url": "https://beta.swissgeol.ch/?layers=ch.swisstopo.geologie-geocover" }'
```

The `Location` header contains the shortened url to retrieve the original url:

```bash
# replace id with the last path segment of the shortened url
curl -v localhost:8080/{id}
```

This returns a response with status `301` and the original url in the `Location` header.

## Endpoints

### `POST /`

Expects a json body of the form:

```json
{
    "url": "http://some.host/..."
}
```

Returns an empty response with status `201 Created` and the shortened url in the `Location` header.

### `GET /{id}`

Returns an empty response with status `301 Moved Permanently` and the original url in the `Location` header.

## Develop

[Install Rust](https://www.rust-lang.org/tools/install) then:

```bash
# clone
git clone git@github.com:swissgeol/abbreviator.git
cd abbreviator

# build and test
cargo test

# run locally
cargo run
```

## Deploy

The following environment variables can be set to customize the service:

| Variable         | Description                                                            |
| ---------------- | ---------------------------------------------------------------------- |
| `DATABASE_URL`   | SQLite database url.                                                   |
| `HOST_WHITELIST` | Whitespace separated list of allowed hosts of the URL to be shortened. |
| `ID_LENGTH`      | Length of the generated key, defaults to `5`.                          |
| `HOST`           | Host the application listens to, defaults to `0.0.0.0`.                |
| `PORT`           | Port the application listens to, defaults to `8080`.                   |

To persist the database create a storage directory and mount it into the container:

```bash
# build
docker build -t abbreviator:prod -f Dockerfile .
# data
mkdir -p storage
# run
docker run -d -p 8080:8080 \
    -e DATABASE_URL='sqlite:///storage/prod.db' \
    -e HOST_WHITELIST='dev.swissgeol.ch int.swissgeol.ch beta.swissgeol.ch swissgeol.ch' \
    -v `pwd`/storage:/storage \
    abbreviator:prod
```

## License

abbreviator is available under the terms of the GNU General Public License Version 3. For full license terms, see [LICENSE](./LICENSE).
