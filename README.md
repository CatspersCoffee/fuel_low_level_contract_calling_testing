# Low Level Contract Call testing

There are two contracts:

```
├── contracts
│   ├── caller
│   └── target
```

We want to call the `target` contract via `caller`, using `call_with_function_selector` in Sway, and `.call_low_level_call()` in the Rust SDK.

## Compile the contracts using:

```console
./0_build_contracts.sh
```

Im using `forc 0.42.1` and as indicated in `fuel-toolchain.toml`.

the CallerContract uses a method straight out of the fuels-rs repo.

The low level caller contract "CallerContract" is the same as:
https://github.com/FuelLabs/fuels-rs/blob/master/packages/fuels/tests/contracts/low_level_caller/src/main.sw

The TargetContract has a few simple methods: set() and get() to set storage variables and a get method to check if the values were stored correctly.

I have annotated the `./tests/harness.rs` file with the methods that work and ones that dont.

To test:

## Start local fuel-core:

Using `fuel-core` version 0.19.1

```console
fuel-core run --ip 127.0.0.1 --port 4000 --db-type in-memory
```
Using the build in wallet:

```Rust
pub const WALLET_FROM_FUEL_CORE: &str =
    "de97d8624a438121b86a1956544bd72ed68cd69f2c99555b08b1e8c51ffd511c";
```


## Deploy contracts:

from inside project root directory.

```console
./1_deploy_contracts.sh
```

OR using cargo:
```console
cargo test --package llcall_testing --test integration_tests -- deploy_contracts --exact --show-output
```

## Test Methods individually:

test_set0_get0 using low level call - This one works:
```console
cargo test --package llcall_testing --test integration_tests -- test_set0_get0 --exact --show-output
```

test_set1_get1 using low level call - This Fails:
```console
cargo test --package llcall_testing --test integration_tests -- test_set1_get1 --exact --show-output
```

test_set2_get2 using low level call - This Fails:
```console
cargo test --package llcall_testing --test integration_tests -- test_set2_get2 --exact --show-output
```

test_set3_get3 using low level call - This Fails:
attempts to populate a struct as call data:
```Rust
DemoStruct {
    a: true,
    b: [1, 2, 3],
    c: 22u64,
}
```

```console
cargo test --package llcall_testing --test integration_tests -- test_set3_get3 --exact --show-output
```


### Check Sanity:

Call set3 directly with a populated DemoStruct and read back - This works.

```console
cargo test --package llcall_testing --test integration_tests -- direct_call_set3_get3 --exact --show-output
```

