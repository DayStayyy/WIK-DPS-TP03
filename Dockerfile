FROM rust:1.64.0 AS build

RUN cargo new --bin WIK-DPS-TP02
WORKDIR /WIK-DPS-TP02

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./Rocket.toml ./Rocket.toml
COPY ./src ./src

RUN cargo build --release

FROM rust:1.64.0

COPY --from=build /WIK-DPS-TP02/target/release/WIK-DPS-TP02 .
COPY ./Rocket.toml ./Rocket.toml
RUN useradd -ms /bin/bash web_user
USER web_user

CMD ["./WIK-DPS-TP02"]