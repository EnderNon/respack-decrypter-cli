cargo build --target x86_64-unknown-linux-musl --package rpfixer --bin rpfixer --release
cargo build --target x86_64-pc-windows-gnu --package rpfixer --bin rpfixer --release

mkdir -p product

cp ./target/x86_64-pc-windows-gnu/release/rpfixer.exe ./product/rpfixer-windows-x64.exe
cp ./target/x86_64-unknown-linux-musl/release/rpfixer ./product/rpfixer-linux-x64

pkgversion=$(cargo metadata --format-version=1 --no-deps | jq '.packages[] | select(.name == "rpfixer") | .version')
echo "package version is:"
echo $pkgversion
balls=${pkgversion::-1}
balls2=${balls:1}
echo $balls2

cd product

zip "rpfixer-windows-x64-${balls2}.zip" rpfixer-windows-x64.exe
zip "rpfixer-linux-x64-${balls2}.zip" rpfixer-linux-x64
