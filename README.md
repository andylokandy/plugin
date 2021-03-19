```
cd tidb_query
cargo build

cd ../tikv
cargo run
```

Output:

```
> cargo run
   Compiling tikv v0.1.0 (/Users/andy/Documents/Code/Rust/plugin/tikv)
    Finished dev [unoptimized + debuginfo] target(s) in 0.58s
     Running `target/debug/tikv`
Host: host plugin system info: PluginBuildInfo {
    api_version: "0.1.0",
    target: "x86_64-apple-darwin",
    host: "x86_64-apple-darwin",
    rustc: "rustc 1.49.0-nightly (1eaadebb3 2020-10-21)",
    target_arch: "x86_64",
}
Host: plugin loaded: tidb_query 0.0.0
Host: handle new request: [Proto] Get Key [107, 49]
Plugin: start to get key [107, 49]
Host: scaning key [107, 49]
Host: returning val of key [107, 49]
Plugin: got val: [91, 118, 97, 108, 93, 32, 111, 102, 32, 107, 49]
Host: coprocessor response: "[Proto] val: [91, 118, 97, 108, 93, 32, 111, 102, 32, 107, 49]"
```