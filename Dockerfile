FROM rust:1.63.0 as development

WORKDIR /app

COPY ./todo-adapter ./todo-adapter
COPY ./todo-app ./todo-app
COPY ./todo-driver ./todo-driver
COPY ./todo-kernel ./todo-kernel
COPY ./Cargo.toml ./Cargo.toml
COPY ./docker-app.env ./.env
