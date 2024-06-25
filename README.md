# Axum and Clean Architecture

An example of web application by using Rust and Axum with Clean Architecture (Explicit Architecture).

## Getting Started

### Setting up database

#### Run Postgresql on Docker

```shell
docker-compose up --build -d
```

or

```shell
docker-compose build --no-cache
docker-compose up -d
```

- `--no-cache`: Do not use cache when building the image.
- `-d`: Detached mode: Run containers in the background, print new container names.

#### Create database and migration in the app container

```shell
docker-compose exec app bash
```
and

```shell
sqlx database create
sqlx migrate run
```

#### Drop database

```shell
sqlx database drop
```

### Run the web application

Execute a command in the running `app` container and `cargo run` in the `app` container.

```shell
docker-compose exec app bash
```

```shell
cargo run
```

### Run the web application (in local)

Create `.env` and `cargo run`.

```shell
cp local.env .env
```

```shell
cargo run
```

### Notes

[docker-compose](https://docs.docker.jp/compose/reference/docker-compose.html)

#### Start, Stop, Restart

```shell
docker-compose start
```

```shell
docker-compose stop
```

```shell
docker-compose restart
```

#### Down

```shell
docker-compose down -v
```

- `-v`: Remove named volumes declared in the `volumes` section of the Compose file and anonymous volumes attached to containers.

#### Exec

Execute a command in a running container.

```shell
docker-compose exec app bash
docker-compose exec db bash
```

## Development

### Dependencies

- Axum 0.7.5
- Postgresql 16

### Architecture

This example project has 4 workspaces:

- todo-driver (driver or controller)
- todo-app (app or usecase)
- todo-kernel (kernel or domain)
- todo-adapter (adapter or infrastructure)

The upper side in this list is to be an upper layer, the lower ones are to be a lower layer.
The upper layers can call or use the lower ones but the opposite calling isn't allowed.
For instance, the driver layer can call the app layer's modules but the app layer cannot call the driver layer's modules.

DIP (Dependency Inversion Principle) is applied between kernel and adapter layer.
For example, the kernel layer's repositories have just definitions of traits, these implementations are in the adapter layer.

The driver layer has only around Axum's definition.
Axum's `Router`, handler and launching the server.
Things around definitions and settings for web applications have to be defined within this layer.

The app layer has a so-called "use case" layer (in the context of clean architecture).
The layer controls the entire application process and logic has to be defined in the range of this layer.

The kernel layer has a so-called "domain" layer (in the context of clean architecture as well).
This layer is the core context of the application.
For instance, calculators for stock stats have to be described within this layer.

The adapter layer has around infrastructure's concerns.
This layer can connect and access outside middlewares, services or APIs.
The access and connection processes have to be bounded in this layer.

See more: 

- [Rust の新しい HTTP サーバーのクレート Axum をフルに活用してサーバーサイドアプリケーション開発をしてみる - Don't Repeat Yourself](https://blog-dry.com/entry/2021/12/26/002649)
- [explicit architecture – @hgraca](https://herbertograca.com/tag/explicit-architecture/)

## License

This project is licensed under the [MIT license](LICENSE).
