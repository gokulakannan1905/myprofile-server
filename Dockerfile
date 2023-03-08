FROM rust:1.67
WORKDIR /app
COPY . . 
RUN cargo build --release
CMD [ "cargo", "run" ]
EXPOSE 80