rm -rf ./target/doc/
cargo doc --no-deps
rm -rf ./docs
echo "<meta http-equiv=\"refresh\" content=\"0; url=tmx_utils\">" > target/doc/index.html
cp -r target/doc ./docs