use std::error::Error;
use wasmtime::{Config, Engine, Linker, Module, Store};
use wasmtime_wasi::p1::{self, WasiP1Ctx};
use wasmtime_wasi::WasiCtxBuilder;
fn main() -> Result<(), Box<dyn Error>> {
    let mut config = Config::new();
    config.wasm_multi_value(true);
    let engine = Engine::new(&config)?;

    let wasm_bytes = include_bytes!("../module.wasm");
    let module = Module::new(&engine, wasm_bytes)?;

    let wasi: WasiP1Ctx = WasiCtxBuilder::new()
        .inherit_stdout()
        .inherit_stderr()
        .build_p1();

    let mut store = Store::new(&engine, wasi);
    let mut linker: Linker<WasiP1Ctx> = Linker::new(&engine);
    p1::add_to_linker_sync(&mut linker, |cx| cx)?;

    let instance = linker.instantiate(&mut store, &module)?;
    let main_fn = instance.get_typed_func::<(), ()>(&mut store, "main")?;
    main_fn.call(&mut store, ())?;

    Ok(())
}