# Axum and Clean Architecture

An example of web application by using Rust and Axum with Clean Architecture (Explicit Architecture).

## Getting Started

### Setting up database tables

Please `createdb` and run SQLs in migrations directory.

`up.sql` can be up tables, `down.sql` removes them.

```shell
createdb todo_db -O <owner_user>
```

### Run the web application

```shell
cargo run
```

## Development

### Dependencies

- Axum
- PostgreSQL 14

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

See more [Explicit Architecture](https://herbertograca.com/tag/explicit-architecture/).

## License

This project is licensed under the [MIT license](LICENSE).
