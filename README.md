# dev-papers

## Building

1. Install rust (wow)
2. Create database and apply all migrations from `dp-web-core/src/migrations`:
```console
$ touch papers.sqlite
$ cat dp-web-core/src/migrations/*.sql | sqlite3 papers.sqlite
```
3. Set `DATABASE_URL` variable and run `cargo build`:
```console
$ DATABASE_URL=sqlite://./papers.sqlite cargo build --release
```
4. Copy `config.example.yml` to `config.yml` and edit it.
5. Run `target/release/dp-web-server`.

## Configuration

See `target/release/dp-web-server`:

```console
API server for hosting papers

Usage: dp-web-server [OPTIONS] <COMMAND>

Commands:
  start          Start the web service
  create-invite  Issue user invite
  help           Print this message or the help of the given subcommand(s)

Options:
  -c, --config <CONFIG>      Path which stores the papers [default: config.yml]
  -d, --database <DATABASE>  Path to sqlite database [default: papers.sqlite]
  -h, --help                 Print help
  -V, --version              Print version
```
