echo "build osx architecture ....."
cargo build --release --target x86_64-apple-darwin
echo "build linux architecture ....."
cargo build --release --target x86_64-unknown-linux-gnu
echo "build windows architecture ....."
cargo build --release --target x86_64-pc-windows-gnu
echo "build nuget packge ....."
cp ./target/x86_64-apple-darwin/release/libNeo_Cryptography_BLS12381_Native.dylib ./runtimes/osx-x64/native/lib
cp ./target/x86_64-apple-darwin/release/libNeo_Cryptography_BLS12381_Native.dylib ./runtimes/osx-arm64/native/lib
cp ./target/x86_64-pc-windows-gnu/release/Neo_Cryptography_BLS12381_Native.dll ./runtimes/win10-x64/native/lib/uap10.0
cp ./target/x86_64-pc-windows-gnu/release/Neo_Cryptography_BLS12381_Native.dll ./runtimes/win10-x86/native/lib/uap10.0
cp ./target/x86_64-unknown-linux-gnu/release/libNeo_Cryptography_BLS12381_Native.so ./runtimes/alpine-x64/native/lib
cp ./target/x86_64-unknown-linux-gnu/release/libNeo_Cryptography_BLS12381_Native.so ./runtimes/linux-x64/native/lib
cp ./target/x86_64-unknown-linux-gnu/release/libNeo_Cryptography_BLS12381_Native.so ./runtimes/linux-arm64/native/lib
cp ./target/x86_64-unknown-linux-gnu/release/libNeo_Cryptography_BLS12381_Native.so ./runtimes/mips64/native/lib
nuget pack