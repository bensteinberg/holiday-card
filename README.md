Holiday Card
============

Docker setup
------------

* `docker-compose up &`
* `docker-compose exec web bash`

Project setup
-------------

* In docker:

  `(cd frontend && npm install)`
  
Development
-----------

In docker:

    `cargo cmd serve`

and visit 127.0.0.1:8000. Frontend and backend should both hot reload. All requests are
served by vue, with some requests proxied through to actix-web, which is running on
port 8001.

Deployment
----------

In docker:

`cargo build --release`

The complete web server binary is in `./target/release/holiday-card`

The binary can be configured with environment variables. Here is the default configuration:

`BIND_ADDR=127.0.0.1:8000 PICTURES_DIR=pictures REINDEX_SECONDS=60 holiday-card`

The above command tells the web server to bind to port 8000 on localhost; serve pictures
from the "pictures" directory (relative or absolute path); and reindex the directory
every 60 seconds.

The binary is not compiled with SSL support, and should be run behind a reverse proxy
like nginx.

### Debian package

In docker:

`cargo deb`






