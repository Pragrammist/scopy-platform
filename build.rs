use std::process::Command;

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();

    // собираем раннер в отдельную target-папку внутри OUT_DIR
    let runner_target_dir = format!("{out_dir}/runner-target");
    let status = Command::new("cargo")
        .args(["build", "--release"])
        .current_dir("runner")
        .env("CARGO_TARGET_DIR", &runner_target_dir)
        .status()
        .expect("failed to spawn cargo for runner");
    assert!(status.success(), "runner build failed");

    // копируем готовый бинарник туда, откуда его ждёт include_bytes!
    let built = format!("{runner_target_dir}/release/wasm-runner");
    let dest = format!("{out_dir}/wasm-runner-bin");
    std::fs::copy(&built, &dest).expect("failed to copy runner");

    println!("cargo:rerun-if-changed=runner/src");
    println!("cargo:rerun-if-changed=runner/Cargo.toml");
}