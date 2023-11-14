use wasmtime::{Config, Engine, component::{Component, Linker}, Store};
use wasmtime_wasi::{preview2::{WasiCtxBuilder, WasiCtx, Table, WasiView, pipe::MemoryOutputPipe}, ambient_authority};

// use guest::Component as GuestComponent;

wasmtime::component::bindgen!("example" in "../wit");


pub struct Data<T> {
    pub inner: T,
    pub wasi: WasiCtx,
    pub table: Table,
}

impl<T: Send> WasiView for Data<T> {
    fn table(&self) -> &wasmtime_wasi::preview2::Table {
        &self.table
    }

    fn table_mut(&mut self) -> &mut wasmtime_wasi::preview2::Table {
        &mut self.table
    }

    fn ctx(&self) -> &WasiCtx {
        &self.wasi
    }

    fn ctx_mut(&mut self) -> &mut WasiCtx {
        &mut self.wasi
    }
}

// #[tokio::main]
fn main() -> Result<(), Box<dyn std::error::Error>>  {
    let mut config = Config::new();
    config.wasm_component_model(true);

    let engine = Engine::new(&config).expect("woot");

    let stdout = MemoryOutputPipe::new(4096);
    let stderr = MemoryOutputPipe::new(4096);

    let wasi = WasiCtxBuilder::new()
        .inherit_stdio()
        .stdout(stdout.clone())
        .stderr(stderr.clone())
        .args(&["test", "."])
        .inherit_network(ambient_authority())
        .allow_ip_name_lookup(true)
        .build();
    
    let mut linker = Linker::new(&engine);

    let _ = wasmtime_wasi::preview2::command::sync::add_to_linker(&mut linker);

    let mut store = Store::new(
        &engine, 
        Data {
        table: wasmtime_wasi::preview2::Table::new(),
        inner: "",
        wasi: wasi,
    },);

    let component1 = Component::from_file(&engine, "../target/wasm32-wasi/debug/guest.wasm").expect("wasm component");
    let (test1, _) = Example::instantiate(&mut store, &component1, &linker).expect("component init");
    let input = vec![b'1'];
    let res = test1.interface0.call_map(&mut store, &input).expect("fn");
    eprintln!("res = {:?}", res);


    Ok(())
}
