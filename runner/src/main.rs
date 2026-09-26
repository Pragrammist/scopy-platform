use std::error::Error;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use wasmtime::{Config, Engine, Linker, Module, Store};
use wasmtime_wasi::p1::{self, WasiP1Ctx};
use wasmtime_wasi::WasiCtxBuilder;

fn main() -> Result<(), Box<dyn Error>> {
    let wasm_bytes = read_embedded_wasm()?;

    let mut config = Config::new();
    config.wasm_multi_value(true);
    let engine = Engine::new(&config)?;

    let module = Module::new(&engine, &wasm_bytes)?;

    let wasi: WasiP1Ctx = WasiCtxBuilder::new()
        .inherit_stdout()
        .inherit_stderr()
        .build_p1();

    let mut store = Store::new(&engine, wasi);
    let mut linker: Linker<WasiP1Ctx> = Linker::new(&engine);
    p1::add_to_linker_sync(&mut linker, |cx| cx)?;

    // start-функция вызовется сама при instantiate
    let _instance = linker.instantiate(&mut store, &module)?;

    Ok(())
}

fn read_embedded_wasm() -> Result<Vec<u8>, Box<dyn Error>> {
    let self_path = std::env::current_exe()?;
    let mut file = File::open(&self_path)?;
    let file_len = file.metadata()?.len();

    // последние 8 байт — длина wasm
    file.seek(SeekFrom::End(-8))?;
    let mut len_bytes = [0u8; 8];
    file.read_exact(&mut len_bytes)?;
    let wasm_len = u64::from_le_bytes(len_bytes) as usize;

    // wasm лежит перед trailer'ом
    let wasm_offset = file_len - 8 - wasm_len as u64;
    file.seek(SeekFrom::Start(wasm_offset))?;
    let mut wasm_bytes = vec![0u8; wasm_len];
    file.read_exact(&mut wasm_bytes)?;

    Ok(wasm_bytes)
}