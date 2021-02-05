use crate::types::Address;
use crate::error::Error;
use crate::log_entry::LogEntry;
use crate::functions::ImportResolver;
use crate::parser;
use crate::provider::Provider;
use crate::runtime::Runtime;
use crate::schedule::Schedule;
use crate::state::State;
use crate::transaction::{Action, Transaction};
use crate::action::{ActionParams, ActionType};
use crate::utils;
use crate::wasm_cost::WasmCosts;
use log::{debug, trace};



pub const IMPORT_MODULE_FN: &str = "seal0";
pub const IMPORT_MODULE_MEMORY: &str = "env";


#[derive(Debug, Clone)]
pub struct ResultData {
    pub gas_left: u64,
    pub data: Vec<u8>,
    pub contract: Address,
    pub logs: Vec<LogEntry>,
}

pub fn execute(
    provider: &mut dyn Provider,
    transaction: &Transaction,
) -> Result<ResultData, Error> {
    let params = match &transaction.action {
        Action::Create(code, salt) => {
            let new_address = utils::contract_address(&transaction.sender, &code, &salt);

            ActionParams {
                code_address: new_address.clone(),
                address: new_address.clone(),
                sender: transaction.sender.clone(),
                origin: transaction.sender.clone(),
                gas: transaction.gas,
                gas_price: transaction.gas_price,
                value: transaction.value,
                action_type: ActionType::Create,
                code: code.clone(),
                args: transaction.args.clone(),
                code_hash: None,
            }
        }
        Action::Call(address) => {
            let acc = provider.account_info(address)?;
            let code = acc.code.clone();
            ActionParams {
                code_address: address.clone(),
                address: address.clone(),
                sender: transaction.sender.clone(),
                origin: transaction.sender.clone(),
                gas: transaction.gas,
                gas_price: transaction.gas_price,
                value: transaction.value,
                action_type: ActionType::Call,
                code: code,
                args: transaction.args.clone(),
                code_hash: None,
            }
        }
    };

    let mut schedule = Schedule::default();
    let wasm = WasmCosts::default();
    schedule.wasm = Some(wasm);

    let module = parser::payload(&params, schedule.wasm())?;
    let loaded_module = wasmi::Module::from_parity_wasm_module(module)?;
    let instantiation_resolver = ImportResolver::with_limit(16, schedule.wasm());

    let mut imports = wasmi::ImportsBuilder::new().with_resolver(IMPORT_MODULE_FN, &instantiation_resolver);

    imports.push_resolver(IMPORT_MODULE_MEMORY, &instantiation_resolver);

    let module_instance = wasmi::ModuleInstance::new(&loaded_module, &imports)?;

    let adjusted_gas = params.gas * u64::from(schedule.wasm().opcodes_div)
        / u64::from(schedule.wasm().opcodes_mul);

    if adjusted_gas > ::std::u64::MAX.into() {
        return Err(Error::Wasm {
            msg: "Wasm interpreter cannot run contracts with gas (wasm adjusted) >= 2^64"
                .to_owned(),
        });
    }

    let initial_memory = instantiation_resolver.memory_size()?;
    trace!(target: "wasm", "Contract requested {:?} pages of initial memory", initial_memory);

    let mut state = State::new(provider);
    let mut runtime = Runtime::new(
        &params,
        &schedule,
        &mut state,
        instantiation_resolver.memory_ref(),
        // cannot overflow, checked above
        adjusted_gas,
    );

    // cannot overflow if static_region < 2^16,
    // initial_memory ∈ [0..2^32)
    // total_charge <- static_region * 2^32 * 2^16
    // total_charge ∈ [0..2^64) if static_region ∈ [0..2^16)
    // qed
    assert!(runtime.schedule().wasm().initial_mem < 1 << 16);


    // TODO: fix me!!!!
    // runtime.charge(|s| initial_memory as u64 * s.wasm().initial_mem as u64)?;

    let instance = module_instance.run_start(&mut runtime)?;
    let invoke_result = instance.invoke_export("call", &[], &mut runtime)?;

    // if let Err(wasmi::Error::Trap(ref trap)) = invoke_result {
    //     if let wasmi::TrapKind::Host(ref boxed) = *trap.kind() {
    //         let ref runtime_err = boxed
    //             .downcast_ref::<Error>()
    //             .expect("Host errors other than runtime::Error never produced; qed");

    //         let mut have_error = false;
    //         match **runtime_err {
    //             Error::Suicide => {
    //                 debug!("Contract suicided.");
    //             }
    //             Error::Return => {
    //                 debug!("Contract returned.");
    //             }
    //             _ => {
    //                 have_error = true;
    //             }
    //         }
    //         if let (true, Err(e)) = (have_error, invoke_result) {
    //             trace!(target: "wasm", "Error executing contract: {:?}", e);
    //             return Err(Error::from(e));
    //         }
    //     }
    // }

    let gas_left = runtime
        .gas_left()
        .expect("Cannot fail since it was not updated since last charge");
    let result = runtime.into_result();
    let gas_left_adj = u64::from(gas_left) * u64::from(schedule.wasm().opcodes_mul)
        / u64::from(schedule.wasm().opcodes_div);

    if result.is_empty() {
        debug!(target: "wasm", "Contract execution result is empty.");
        Ok(ResultData {
            gas_left: gas_left_adj,
            data: vec![],
            contract: params.address,
            // TODO::::: logs????
            logs: vec![], // ext.logs().to_vec(),
        })
    } else {
        if let Action::Create(_, _) = &transaction.action {
            runtime.init_code(params.address, result.to_vec());
        }

        runtime.update_state()?;

        Ok(ResultData {
            gas_left: gas_left_adj,
            data: result.to_vec(),
            contract: params.address,
            // TODO::::: logs????
            logs: vec![], // ext.logs().to_vec(),
        })
    }
}


