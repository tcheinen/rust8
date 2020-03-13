#!/bin/sh
cargo doc --no-deps
echo "<meta http-equiv=\"refresh\" content=\"0; url=rust8_core/index.html\" />" > target/doc/index.html
