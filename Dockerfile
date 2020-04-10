FROM rust:1-slim
COPY target/release/spiritwood-server /spiritwood-server
EXPOSE 8000
ENV ROCKET_LOG normal
CMD ["/spiritwood-server"]
