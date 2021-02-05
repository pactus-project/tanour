use crate::error::Error;
use crate::action::ActionParams;
use crate::wasm_cost::WasmCosts;
use parity_wasm::elements::{self, Deserialize};
use pwasm_utils::{self, rules};

fn gas_rules(wasm_costs: &WasmCosts) -> rules::Set {
    rules::Set::new(wasm_costs.regular, {
        let mut vals = ::std::collections::BTreeMap::new();
        vals.insert(
            rules::InstructionType::Load,
            rules::Metering::Fixed(wasm_costs.mem as u32),
        );
        vals.insert(
            rules::InstructionType::Store,
            rules::Metering::Fixed(wasm_costs.mem as u32),
        );
        vals.insert(
            rules::InstructionType::Div,
            rules::Metering::Fixed(wasm_costs.div as u32),
        );
        vals.insert(
            rules::InstructionType::Mul,
            rules::Metering::Fixed(wasm_costs.mul as u32),
        );
        vals
    })
    .with_grow_cost(wasm_costs.grow_mem)
    .with_forbidden_floats()
}

/// Splits payload to code and data according to params_type, also
/// loads the module instance from payload and injects gas counter according
/// to schedule.
pub fn payload<'a>(
    params: &'a ActionParams,
    wasm_costs: &WasmCosts,
) -> Result<elements::Module, Error> {
    let mut cursor = ::std::io::Cursor::new(&params.code[..]);

    let deserialized_module =
        elements::Module::deserialize(&mut cursor).map_err(|err| Error::Wasm {
            msg: format!("Error deserializing contract code ({:?})", err),
        })?;

    if deserialized_module
        .memory_section()
        .map_or(false, |ms| ms.entries().len() > 0)
    {
        // According to WebAssembly spec, internal memory is hidden from embedder and should not
        // be interacted with. So we disable this kind of modules at decoding level.
        return Err(Error::Wasm {
            msg: format!("Malformed wasm module: internal memory"),
        });
    }

    let contract_module =
        pwasm_utils::inject_gas_counter(deserialized_module, &gas_rules(wasm_costs), "env").map_err(
            |_| Error::Wasm {
                msg: format!("Wasm contract error: bytecode invalid"),
            },
        )?;

    let contract_module =
        pwasm_utils::stack_height::inject_limiter(contract_module, wasm_costs.max_stack_height)
            .map_err(|_| Error::Wasm {
                msg: format!("Wasm contract error: stack limiter failure"),
            })?;

    Ok(contract_module)
}
