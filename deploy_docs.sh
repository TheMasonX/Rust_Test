rm -rf ./docs
echo "<meta http-equiv=\"refresh\" content=\"0; url=test_project\">" > target/doc/index.html
cp -r target/doc ./docs