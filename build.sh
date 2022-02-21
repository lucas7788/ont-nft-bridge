RUSTFLAGS="-C link-arg=-zstack-size=32768" cargo build --release --target wasm32-unknown-unknown
cd ./target/wasm32-unknown-unknown/release
ontio-wasm-build oep5_receiver.wasm
ontio-wasm-build bridge.wasm
mv oep5_receiver_optimized.wasm oep5_receiver_optimized1.wasm
cd ../../../

sed -i -e 's/unsupported action/unsupported action2/' ./contracts/oep5-receiver/src/lib.rs
rm ./contracts/oep5-receiver/src/lib.rs-e
RUSTFLAGS="-C link-arg=-zstack-size=32768" cargo build --release --target wasm32-unknown-unknown
cd ./target/wasm32-unknown-unknown/release
ontio-wasm-build oep5_receiver.wasm
mv oep5_receiver_optimized.wasm oep5_receiver_optimized2.wasm
cd ../../../


sed -i -e 's/unsupported action2/unsupported action3/' ./contracts/oep5-receiver/src/lib.rs
rm ./contracts/oep5-receiver/src/lib.rs-e
RUSTFLAGS="-C link-arg=-zstack-size=32768" cargo build --release --target wasm32-unknown-unknown
cd ./target/wasm32-unknown-unknown/release
ontio-wasm-build oep5_receiver.wasm
mv oep5_receiver_optimized.wasm oep5_receiver_optimized3.wasm
cd ../../../

sed -i -e 's/unsupported action3/unsupported action4/' ./contracts/oep5-receiver/src/lib.rs
rm ./contracts/oep5-receiver/src/lib.rs-e
RUSTFLAGS="-C link-arg=-zstack-size=32768" cargo build --release --target wasm32-unknown-unknown
cd ./target/wasm32-unknown-unknown/release
ontio-wasm-build oep5_receiver.wasm
mv oep5_receiver_optimized.wasm oep5_receiver_optimized4.wasm
cd ../../../

sed -i -e 's/unsupported action4/unsupported action/' ./contracts/oep5-receiver/src/lib.rs
rm ./contracts/oep5-receiver/src/lib.rs-e