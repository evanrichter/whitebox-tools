# Build Stage
FROM ghcr.io/evanrichter/cargo-fuzz:latest as builder

## Add source code to the build stage.
ADD . /src
WORKDIR /src

# remove this when nalgebra builds on rust nightly
RUN rustup install stable && rustup default stable

RUN echo building instrumented harnesses && \
    # remove this when nalgebra builds on rust nightly
    export RUSTC_BOOTSTRAP=1 && \
    bash -c "pushd whitebox-raster/fuzz && cargo fuzz build && popd" && \
    mv whitebox-raster/fuzz/target/x86_64-unknown-linux-gnu/release/geotiff /geotiff && \
    echo done

RUN echo building non-instrumented harnesses && \
    # remove this when nalgebra builds on rust nightly
    export RUSTC_BOOTSTRAP=1 && \
    export RUSTFLAGS="--cfg fuzzing -Clink-dead-code -Cdebug-assertions -C codegen-units=1" && \
    bash -c "pushd whitebox-raster/fuzz && cargo build --release && popd" && \
    mv whitebox-raster/fuzz/target/release/geotiff /geotiff_no_inst && \
    echo done

# Package Stage
FROM rustlang/rust:nightly

COPY --from=builder /geotiff /geotiff_no_inst /
