# Rust Quickstart Project

To start the server, first run

```bash
$ docker compose up -d 
```

to start the development database, then create a new `config.json`:

```json
{
    "database_url": "postgres://application:P@ssw0rd@localhost:5432/db"
}
```
