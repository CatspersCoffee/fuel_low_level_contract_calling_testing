contract;

// Stright from: https://github.com/FuelLabs/fuels-rs/blob/master/packages/fuels/tests/contracts/low_level_caller/src/main.sw

use std::constants::BASE_ASSET_ID;
use std::low_level_call::{call_with_function_selector, CallParams};
use std::bytes::Bytes;

abi CallerContract {
    fn call_low_level_call(target: ContractId, function_selector: Bytes, calldata: Bytes, single_value_type_arg: bool);
}

impl CallerContract for Contract {
    // ANCHOR: low_level_call_contract
    fn call_low_level_call(
        target: ContractId,
        function_selector: Bytes,
        calldata: Bytes,
        single_value_type_arg: bool,
    ) {
        let call_params = CallParams {
            coins: 0,
            asset_id: BASE_ASSET_ID,
            gas: 100_000,
        };

        call_with_function_selector(target, function_selector, calldata, single_value_type_arg, call_params);
    }
    // ANCHOR_END: low_level_call_contract
}
