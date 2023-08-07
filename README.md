# minirepro-libserde-multivalue-return

Very simple repo demonstreating that building serde_json fails when the flag multivalue is turned on.

Background: WASM supports multi-value returns, for example, 

Test it with:

    RUSTFLAGS="-C target-feature=+multivalue" cargo build --target=wasm32-unknown-unknown

Or with:

    RUSTFLAGS="-C target-feature=+multivalue" cargo wasi build

The error I get on my system is...

MacBook-Pro:minirepro-libserde-multivalue-return adam$ RUSTFLAGS="-C target-feature=+multivalue" cargo build --target=wasm32-unknown-unknown
   Compiling ryu v1.0.15
   Compiling itoa v1.0.9
   Compiling serde v1.0.183
   Compiling serde_json v1.0.104
   Compiling minirepro-libserde-multivalue-return v0.1.0 (/Users/adam/minirepro-libserde-multivalue-return)
error: linking with `rust-lld` failed: exit status: 1
  |
  = note: LC_ALL="C" PATH="/Users/adam/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/x86_64-apple-darwin/bin:/Users/adam/.wasmtime/bin:/Library/Frameworks/Python.framework/Versions/3.11/bin:/Users/adam/.cargo/bin:/Users/adam/google-cloud-sdk/bin:/Library/Frameworks/Python.framework/Versions/3.10/bin:/Users/adam/wabt/build:/usr/local/bin:/usr/local/bin:/System/Cryptexes/App/usr/bin:/usr/bin:/bin:/usr/sbin:/sbin:/usr/local/go/bin:/usr/local/share/dotnet:/opt/X11/bin:/Library/Frameworks/Mono.framework/Versions/Current/Commands:/var/run/com.apple.security.cryptexd/codex.system/bootstrap/usr/local/bin:/var/run/com.apple.security.cryptexd/codex.system/bootstrap/usr/bin:/var/run/com.apple.security.cryptexd/codex.system/bootstrap/usr/appleinternal/bin:/usr/local/bin" VSLANG="1033" "rust-lld" "-flavor" "wasm" "--rsp-quoting=posix" "--export" "flip" "--export=__heap_base" "--export=__data_end" "-z" "stack-size=1048576" "--stack-first" "--allow-undefined" "--fatal-warnings" "--no-demangle" "--no-entry" "/Users/adam/minirepro-libserde-multivalue-return/target/wasm32-unknown-unknown/debug/deps/minirepro_libserde_multivalue_return.33ysn1yzorsl5z8s.rcgu.o" "/Users/adam/minirepro-libserde-multivalue-return/target/wasm32-unknown-unknown/debug/deps/minirepro_libserde_multivalue_return.en537ocuv57wej9.rcgu.o" "-L" "/Users/adam/minirepro-libserde-multivalue-return/target/wasm32-unknown-unknown/debug/deps" "-L" "/Users/adam/minirepro-libserde-multivalue-return/target/debug/deps" "-L" "/Users/adam/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/wasm32-unknown-unknown/lib" "/Users/adam/minirepro-libserde-multivalue-return/target/wasm32-unknown-unknown/debug/deps/libserde_json-91e5a43084480c19.rlib" "/Users/adam/minirepro-libserde-multivalue-return/target/wasm32-unknown-unknown/debug/deps/libryu-618c3c87e3c23d57.rlib" "/Users/adam/minirepro-libserde-multivalue-return/target/wasm32-unknown-unknown/debug/deps/libitoa-fcfa127f81d6bf94.rlib" "/Users/adam/minirepro-libserde-multivalue-return/target/wasm32-unknown-unknown/debug/deps/libserde-561d5cdc3119e0a7.rlib" "/Users/adam/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/wasm32-unknown-unknown/lib/libstd-d66a635e2d91bd07.rlib" "/Users/adam/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/wasm32-unknown-unknown/lib/libpanic_abort-9c23ad0f53420e97.rlib" "/Users/adam/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/wasm32-unknown-unknown/lib/libdlmalloc-b7a70608addd07ea.rlib" "/Users/adam/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/wasm32-unknown-unknown/lib/librustc_demangle-93bf8bf53eba70fb.rlib" "/Users/adam/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/wasm32-unknown-unknown/lib/libstd_detect-35b4cce68a99a82c.rlib" "/Users/adam/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/wasm32-unknown-unknown/lib/libhashbrown-d95c35360ed0ef85.rlib" "/Users/adam/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/wasm32-unknown-unknown/lib/libminiz_oxide-725f158fe0f54278.rlib" "/Users/adam/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/wasm32-unknown-unknown/lib/libadler-227b8ebb645db194.rlib" "/Users/adam/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/wasm32-unknown-unknown/lib/librustc_std_workspace_alloc-e40f58c341ac25af.rlib" "/Users/adam/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/wasm32-unknown-unknown/lib/libunwind-c69db5a7bc5f2025.rlib" "/Users/adam/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/wasm32-unknown-unknown/lib/libcfg_if-768266d8ab70fd69.rlib" "/Users/adam/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/wasm32-unknown-unknown/lib/liblibc-f8634d278ed8ebda.rlib" "/Users/adam/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/wasm32-unknown-unknown/lib/liballoc-7be653f7696a71f0.rlib" "/Users/adam/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/wasm32-unknown-unknown/lib/librustc_std_workspace_core-1a010c65bca775b5.rlib" "/Users/adam/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/wasm32-unknown-unknown/lib/libcore-658cdd8d8c2a0952.rlib" "/Users/adam/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/wasm32-unknown-unknown/lib/libcompiler_builtins-94abef8d4c58c981.rlib" "-L" "/Users/adam/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/wasm32-unknown-unknown/lib" "-o" "/Users/adam/minirepro-libserde-multivalue-return/target/wasm32-unknown-unknown/debug/deps/minirepro_libserde_multivalue_return.wasm" "--gc-sections" "--no-entry" "-O0"
  = note: rust-lld: error: function signature mismatch: _ZN60_$LT$std..io..error..Error$u20$as$u20$core..error..Error$GT$6source17h2ce36b5e3d976d28E
          >>> defined as (i32) -> i32 in /Users/adam/minirepro-libserde-multivalue-return/target/wasm32-unknown-unknown/debug/deps/libserde_json-91e5a43084480c19.rlib(serde_json-91e5a43084480c19.serde_json.fbc0da11-cgu.3.rcgu.o)
          >>> defined as (i32, i32) -> void in /Users/adam/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/wasm32-unknown-unknown/lib/libstd-d66a635e2d91bd07.rlib(std-d66a635e2d91bd07.std.148725e7-cgu.0.rcgu.o)
          
          rust-lld: error: function signature mismatch: _ZN4core5slice6memchr14memchr_aligned17ha1de217aab31511aE
          >>> defined as (i32, i32, i32) -> i32 in /Users/adam/minirepro-libserde-multivalue-return/target/wasm32-unknown-unknown/debug/deps/libserde_json-91e5a43084480c19.rlib(serde_json-91e5a43084480c19.serde_json.fbc0da11-cgu.1.rcgu.o)
          >>> defined as (i32, i32, i32, i32) -> void in /Users/adam/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/wasm32-unknown-unknown/lib/libcore-658cdd8d8c2a0952.rlib(core-658cdd8d8c2a0952.core.a4eb9613-cgu.0.rcgu.o)
        
