use anyhow::Result;
use wasmtime::{Caller, Engine, Linker, Module, Store};

#[tokio::main]
async fn main() -> Result<()> {
    let engine = Engine::default();

    let wat = r#"
        (module
            (import "host" "host_func" (func $host_hello (param i32)))

            (func (export "hello")
                i32.const 3
                call $host_hello)
        )
    "#;

    let module = Module::new(&engine, wat)?;

    let mut linker = Linker::new(&engine);

    linker.func_wrap("host", "host_func", |caller: Caller<'_, u32>, param: i32| {
        println!("Got {} from WebAssembly", param);
        println!("my host state is: {}", caller.data());
    })?;

    let mut store = Store::new(&engine, 4);

    let instance = linker.instantiate(&mut store, &module)?;
    let hello = instance.get_typed_func::<(), ()>(&mut store, "hello")?;

    hello.call(&mut store, ())?;

    Ok(())
}
