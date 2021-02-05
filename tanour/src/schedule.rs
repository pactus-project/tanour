use crate::wasm_cost::WasmCosts;
use primitive_types::U256;

/// Definition of the cost schedule and other parameterisations for the EVM.
#[derive(Debug)]
pub struct Schedule {
    /// Does it support exceptional failed code deposit
    pub exceptional_failed_code_deposit: bool,
    /// Does it have a delegate cal
    pub have_delegate_call: bool,
    /// Does it have a CREATE2 instruction
    pub have_create2: bool,
    /// Does it have a REVERT instruction
    pub have_revert: bool,
    /// Does it have a EXTCODEHASH instruction
    pub have_extcodehash: bool,
    /// VM stack limit
    pub stack_limit: usize,
    /// Max number of nested calls/creates
    pub max_depth: usize,
    /// Gas prices for instructions in all tiers
    pub tier_step_gas: [usize; 8],
    /// Gas price for `EXP` opcode
    pub exp_gas: usize,
    /// Additional gas for `EXP` opcode for each byte of exponent
    pub exp_byte_gas: usize,
    /// Gas price for `SHA3` opcode
    pub sha3_gas: usize,
    /// Additional gas for `SHA3` opcode for each word of hashed memory
    pub sha3_word_gas: usize,
    /// Gas price for loading from storage
    pub sload_gas: usize,
    /// Special gas price for dirty gas of SSTORE, after net gas metering.
    pub sstore_dirty_gas: Option<usize>,
    /// Gas price for setting new value to storage (`storage==0`, `new!=0`)
    pub sstore_set_gas: usize,
    /// Gas price for altering value in storage
    pub sstore_reset_gas: usize,
    /// Gas refund for `SSTORE` clearing (when `storage!=0`, `new==0`)
    pub sstore_refund_gas: usize,
    /// Gas price for `JUMPDEST` opcode
    pub jumpdest_gas: usize,
    /// Gas price for `LOG*`
    pub log_gas: usize,
    /// Additional gas for data in `LOG*`
    pub log_data_gas: usize,
    /// Additional gas for each topic in `LOG*`
    pub log_topic_gas: usize,
    /// Gas price for `CREATE` opcode
    pub create_gas: usize,
    /// Gas price for `*CALL*` opcodes
    pub call_gas: usize,
    /// Stipend for transfer for `CALL|CALLCODE` opcode when `value>0`
    pub call_stipend: usize,
    /// Additional gas required for value transfer (`CALL|CALLCODE`)
    pub call_value_transfer_gas: usize,
    /// Additional gas for creating new account (`CALL|CALLCODE`)
    pub call_new_account_gas: usize,
    /// Refund for SUICIDE
    pub suicide_refund_gas: usize,
    /// Gas for used memory
    pub memory_gas: usize,
    /// Coefficient used to convert memory size to gas price for memory
    pub quad_coeff_div: usize,
    /// Cost for contract length when executing `CREATE`
    pub create_data_gas: usize,
    /// Maximum code size when creating a contract.
    pub create_data_limit: usize,
    /// Transaction cost
    pub tx_gas: usize,
    /// `CREATE` transaction cost
    pub tx_create_gas: usize,
    /// Additional cost for empty data transaction
    pub tx_data_zero_gas: usize,
    /// Additional cost for non-empty data transaction
    pub tx_data_non_zero_gas: usize,
    /// Gas price for copying memory
    pub copy_gas: usize,
    /// Price of EXTCODESIZE
    pub extcodesize_gas: usize,
    /// Base price of EXTCODECOPY
    pub extcodecopy_base_gas: usize,
    /// Price of BALANCE
    pub balance_gas: usize,
    /// Price of EXTCODEHASH
    pub extcodehash_gas: usize,
    /// Price of SUICIDE
    pub suicide_gas: usize,
    /// Amount of additional gas to pay when SUICIDE credits a non-existant account
    pub suicide_to_new_account_cost: usize,
    /// If Some(x): let limit = GAS * (x - 1) / x; let CALL's gas = min(requested, limit). let CREATE's gas = limit.
    /// If None: let CALL's gas = (requested > GAS ? [OOG] : GAS). let CREATE's gas = GAS
    pub sub_gas_cap_divisor: Option<usize>,
    /// Don't ever make empty accounts; contracts start with nonce=1. Also, don't charge 25k when sending/suicide zero-value.
    pub no_empty: bool,
    /// Kill empty accounts if touched.
    pub kill_empty: bool,
    /// Blockhash instruction gas cost.
    pub blockhash_gas: usize,
    /// Static Call opcode enabled.
    pub have_static_call: bool,
    /// RETURNDATA and RETURNDATASIZE opcodes enabled.
    pub have_return_data: bool,
    /// SHL, SHR, SAR opcodes enabled.
    pub have_bitwise_shifting: bool,
    /// CHAINID opcode enabled.
    pub have_chain_id: bool,
    /// SELFBALANCE opcode enabled.
    pub have_selfbalance: bool,
    /// Kill basic accounts below this balance if touched.
    pub kill_dust: CleanDustMode,
    /// Enable EIP-1283 rules
    pub eip1283: bool,
    /// Enable EIP-1706 rules
    pub eip1706: bool,
    /// Latest VM version for contract creation transaction.
    pub latest_version: U256,
    /// Wasm extra schedule settings, if wasm activated
    pub wasm: Option<WasmCosts>,
}

#[allow(dead_code)]
/// Dust accounts cleanup mode.
#[derive(Debug, PartialEq, Eq)]
pub enum CleanDustMode {
    /// Dust cleanup is disabled.
    Off,
    /// Basic dust accounts will be removed.
    BasicOnly,
    /// Basic and contract dust accounts will be removed.
    WithCodeAndStorage,
}

impl Schedule {
    /// Schedule for the Frontier-era of the Ethereum main net.
    pub fn new_frontier() -> Schedule {
        Self::new(false, false, 21000)
    }

    fn new(efcd: bool, hdc: bool, tcg: usize) -> Schedule {
        Schedule {
            exceptional_failed_code_deposit: efcd,
            have_delegate_call: hdc,
            have_create2: false,
            have_revert: false,
            have_return_data: false,
            have_bitwise_shifting: false,
            have_chain_id: false,
            have_selfbalance: false,
            have_extcodehash: false,
            stack_limit: 1024,
            max_depth: 1024,
            tier_step_gas: [0, 2, 3, 5, 8, 10, 20, 0],
            exp_gas: 10,
            exp_byte_gas: 10,
            sha3_gas: 30,
            sha3_word_gas: 6,
            sload_gas: 50,
            sstore_dirty_gas: None,
            sstore_set_gas: 20000,
            sstore_reset_gas: 5000,
            sstore_refund_gas: 15000,
            jumpdest_gas: 1,
            log_gas: 375,
            log_data_gas: 8,
            log_topic_gas: 375,
            create_gas: 32000,
            call_gas: 40,
            call_stipend: 2300,
            call_value_transfer_gas: 9000,
            call_new_account_gas: 25000,
            suicide_refund_gas: 24000,
            memory_gas: 3,
            quad_coeff_div: 512,
            create_data_gas: 200,
            create_data_limit: usize::max_value(),
            tx_gas: 21000,
            tx_create_gas: tcg,
            tx_data_zero_gas: 4,
            tx_data_non_zero_gas: 68,
            copy_gas: 3,
            extcodesize_gas: 20,
            extcodecopy_base_gas: 20,
            extcodehash_gas: 400,
            balance_gas: 20,
            suicide_gas: 0,
            suicide_to_new_account_cost: 0,
            sub_gas_cap_divisor: None,
            no_empty: false,
            kill_empty: false,
            blockhash_gas: 20,
            have_static_call: false,
            kill_dust: CleanDustMode::Off,
            eip1283: false,
            eip1706: false,
            latest_version: U256::zero(),
            wasm: None,
        }
    }

    /// Returns wasm schedule
    ///
    /// May panic if there is no wasm schedule
    pub fn wasm(&self) -> &WasmCosts {
        // *** Prefer PANIC here instead of silently breaking consensus! ***
        self.wasm.as_ref().expect(
            "Wasm schedule expected to exist while checking wasm contract. Misconfigured client?",
        )
    }
}

impl Default for Schedule {
    fn default() -> Self {
        Schedule::new_frontier()
    }
}
