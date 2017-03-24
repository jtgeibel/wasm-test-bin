source ~/src/emsdk_portable/emsdk_env.sh
emsdk activate sdk-incoming-64bit

cargo build --target=wasm32-unknown-emscripten

mv target/wasm32-unknown-emscripten/debug/deps/wasm_test_bin-*.asm.js output/wasm_test_bin.asm.js
mv target/wasm32-unknown-emscripten/debug/deps/wasm_test_bin-*.js output/wasm_test_bin.js
mv target/wasm32-unknown-emscripten/debug/deps/wasm_test_bin-*.wast output/wasm_test_bin.wast

git add .
