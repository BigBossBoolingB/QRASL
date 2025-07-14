use crate::primitives::{Address, Transaction};
use crate::state::StateMachine;
use std::collections::HashMap;
use wasmi::{
    Caller, Engine, Extern, Func, Linker, Memory, MemoryType, Module, Mutability, Store, Value,
};

pub fn execute_contract(
    state_machine: &mut StateMachine,
    tx: &Transaction,
) -> Result<(), &'static str> {
    let contract_code = state_machine
        .contract_codes
        .get(&tx.recipient)
        .ok_or("Contract code not found")?;

    let engine = Engine::default();
    let module = Module::new(&engine, &contract_code[..]).map_err(|_| "Failed to create module")?;
    let storage = state_machine
        .contract_storage
        .entry(tx.recipient)
        .or_insert_with(HashMap::new);

    let mut store = Store::new(&engine, storage);
    let memory_type = MemoryType::new(1, None);
    let memory = Memory::new(&mut store, memory_type).unwrap();
    let mut linker = Linker::new(&engine);

    let set_storage = Func::wrap(
        &mut store,
        |mut caller: Caller<'_, HashMap<[u8; 32], [u8; 32]>>, key_ptr: u32, value_ptr: u32| {
            let mut key = [0u8; 32];
            let mut value = [0u8; 32];
            let mem = caller.get_export("memory").unwrap().into_memory().unwrap();
            mem.read(&caller, key_ptr as usize, &mut key).unwrap();
            mem.read(&caller, value_ptr as usize, &mut value).unwrap();

            caller.data_mut().insert(key, value);
        },
    );

    linker.define("env", "set_storage", set_storage).unwrap();
    let instance = linker.instantiate(&mut store, &module).unwrap().start(&mut store).unwrap();
    let entry_point = instance
        .get_export(&mut store, "main")
        .and_then(|e| e.into_func())
        .ok_or("'main' function not found in contract")?;

    entry_point
        .call(&mut store, &[], &mut [])
        .map_err(|_| "Contract execution failed")?;

    Ok(())
}
