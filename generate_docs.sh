cargo doc --no-deps
rm -rf ./docs
echo "<meta http-equiv=\"refresh\" content=\"0; url=jadis\">" > target/doc/index.html
echo "jadis.tahar.dev" > target/doc/CNAME
cp -r target/doc ./docs
