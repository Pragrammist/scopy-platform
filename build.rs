use std::process::Command;
use std::env;
use std::fs;
use std::path::Path;


fn main() {
    build_runner();
    include_build_in_modules();
}



fn build_runner(){
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

fn include_build_in_modules(){
    let modules_dir = Path::new("src/build_in_modules");
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest = Path::new(&out_dir).join("builtin_modules.rs");

    let mut files: Vec<_> = fs::read_dir(modules_dir)
        .expect("failed to read src/build_in_modules")
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map(|x| x == "js").unwrap_or(false))
        .collect();
    files.sort_by_key(|e| e.path());

    let mut generated = String::new();
    generated.push_str("vec![\n");

    for entry in files {
        let path = entry.path();
        let name = path.file_name().unwrap().to_str().unwrap();
        let abs = fs::canonicalize(&path).unwrap();

        generated.push_str(&format!(
            "    CodeModuleMetaData {{\n\
             \x20       code: include_str!({abs:?}).to_string(),\n\
             \x20       name: {name:?}.to_string(),\n\
             \x20       module_meta_type: CodeModuleSourceFileType::Internal,\n\
             \x20   }},\n",
            abs = abs.to_str().unwrap(),
            name = name,
        ));

        println!("cargo:rerun-if-changed={}", path.display());
    }

    generated.push_str("]\n");

    fs::write(&dest, generated).expect("failed to write builtin_modules.rs");

    println!("cargo:rerun-if-changed=src/build_in_modules");    
}