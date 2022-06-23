FROM rust:1.61-alpine

WORKDIR /usr/src/tcbscans
COPY src/ src/

RUN 
