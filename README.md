# A quick starter backend application with Rust

### What is the objective of this project?

This project was built to help quickstart Rust projects for backend servers.

### Feature set of the backend server

- [x] REST API routes
- [x] Connection to a DB
- [x] Logging
- [ ] Docker deployment

### Developer notes

#### Dependency and linker setup for libpql (postgres C library)

On MacOS, the `LIBRARY_PATH` environment variable must be set.

```commandline
export LIBRARY_PATH=/usr/local/opt/libpq/lib
```

#### Connection to local database

The `DATABASE_URL` environment variable must be set.

```commandline
export DATABASE_URL=postgres://username:password@localhost:5432/diesel_demo
```
