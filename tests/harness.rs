use fuels::{prelude::*, types::{ContractId, Identity}};
use rand::prelude::{Rng};

use std::str::FromStr;
use std::fs::File;
use std::io::{self, BufRead, Write};

use fuels::{
    core::{
        codec::{calldata, fn_selector},
    },
    accounts::fuel_crypto::SecretKey,
};

use fuels::{
    types::Bits256,
};

pub const BASE_ASSET_ID: AssetId = AssetId::BASE;
const RPC: &str = "127.0.0.1:4000";

abigen!(Contract(
    name = "CallerContract",
    abi = "./contracts/caller/out/debug/caller-abi.json"
    ),
    Contract(
        name = "TargetContract",
        abi = "./contracts/target/out/debug/target-abi.json"
    )
);

pub const CALLER_CONTRACT_BINARY_PATH: &str =
    "./contracts/caller/out/debug/caller.bin";
pub const TARGET_CONTRACT_BINARY_PATH: &str =
    "./contracts/target/out/debug/target.bin";

pub const WALLET_FROM_FUEL_CORE: &str =
    "de97d8624a438121b86a1956544bd72ed68cd69f2c99555b08b1e8c51ffd511c";


//--------------------------------------------------------------------------------------

///
/// Deploys TargetContract
///
async fn _deploy_target_contract() {

    let _provider = match Provider::connect(RPC).await {
        Ok(p) => p,
        Err(error) => panic!("❌ Problem creating provider: {:#?}", error),
    };

    // get wallet0
    let secret0 = WALLET_FROM_FUEL_CORE;
    let wallet0 = WalletUnlocked::new_from_private_key(
        SecretKey::from_str(&secret0).unwrap(),
        Some(_provider.clone())
    );

    println!("wallet0 address (hex) \t: 0x{}", Address::from(wallet0.address()));
    // let balances0 = _provider.get_balances(wallet0.address()).await;
    // println!("Address 0 balances: {:#?}", balances0 );

    // deploy with salt:
    let mut rng = rand::thread_rng();
    let salt = rng.gen::<[u8; 32]>();
    //println!("salt = {}", hex::encode(salt));

    let configuration = LoadConfiguration::default()
        .set_salt(salt);
    let c_id = Contract::load_from(
        TARGET_CONTRACT_BINARY_PATH,
        configuration,)
        .unwrap()
        .deploy(&wallet0, TxParameters::default())
        .await;

    let contract_id = match c_id {
        Ok(contractid) => contractid,
        Err(error) => panic!("❌ Problem deploying the contract: {:#?}", error),
    };
    println!("TargetContract Contract deployed @ {contract_id}");
    println!("ID bech32 \t: {}", contract_id.clone().to_string());
    let tcid: ContractId = contract_id.clone().into();
    println!("ID (hex) \t: {}", tcid);
    write_cid_to_file("target_cid.txt".to_string(), tcid);

}

#[tokio::test]
async fn deploy_target() {
    _deploy_target_contract().await;
}
// cargo test --package llcall_testing --test integration_tests -- deploy_target --exact --show-output

///
/// Deploys CallerContract
///
async fn _deploy_caller_contract() {

    let _provider = match Provider::connect(RPC).await {
        Ok(p) => p,
        Err(error) => panic!("❌ Problem creating provider: {:#?}", error),
    };

    // get wallet0
    let secret0 = WALLET_FROM_FUEL_CORE;
    let wallet0 = WalletUnlocked::new_from_private_key(
        SecretKey::from_str(&secret0).unwrap(),
        Some(_provider.clone())
    );

    //println!("wallet0 address (hex) \t: 0x{}", Address::from(wallet0.address()));
    //let balances0 = _provider.get_balances(wallet0.address()).await;
    //println!("Address 0 balances: {:#?}", balances0 );

    // deploy with salt:
    let mut rng = rand::thread_rng();
    let salt = rng.gen::<[u8; 32]>();
    //println!("salt = {}", hex::encode(salt));

    let configuration = LoadConfiguration::default()
        .set_salt(salt);
    let c_id = Contract::load_from(
        CALLER_CONTRACT_BINARY_PATH,
        configuration,)
        .unwrap()
        .deploy(&wallet0, TxParameters::default())
        .await;

    let contract_id = match c_id {
        Ok(contractid) => contractid,
        Err(error) => panic!("❌ Problem deploying the contract: {:#?}", error),
    };
    println!("CallerContract Contract deployed @ {contract_id}");
    println!("ID bech32 \t: {}", contract_id.clone().to_string());
    let ccid: ContractId = contract_id.clone().into();
    println!("ID (hex) \t: {}", ccid);
    write_cid_to_file("caller_cid.txt".to_string(), ccid);

}

#[tokio::test]
async fn deploy_caller_contract() {
    _deploy_caller_contract().await;
}
// cargo test --package llcall_testing --test integration_tests -- deploy_caller_contract --exact --show-output


//-----------------------------------------------------------------------
//SECTION - Deploy Contracts:
#[tokio::test]
async fn deploy_contracts() {
    print!("\n");
    _deploy_caller_contract().await;
    print!("\n");
    _deploy_target_contract().await;

}
// cargo test --package llcall_testing --test integration_tests -- deploy_contracts --exact --show-output




//-----------------------------------------------------------------------
//SECTION - TESTS:

///
/// # set0() and get0() -> This test succeeds as expected.
///
///#ANCHOR - Tests the TargetContract methods set0() and get0() via low level call.
#[tokio::test]
async fn test_set0_get0() {
    println!("Set the owners address in the CA via the EP:");

    let provider = match Provider::connect(RPC).await {
        Ok(p) => p,
        Err(error) => panic!("❌ Problem creating provider: {:#?}", error),
    };

    //------------------------------------
    // get CallerContract Contract ID from caller_cid.txt file &
    // get TargetContract Contract ID from target_cid.txt file:

    let callercontract_cid: ContractId = read_cid_from_file("caller_cid.txt".to_string()).unwrap();
    let targetcontract_id: ContractId = read_cid_from_file("target_cid.txt".to_string()).unwrap();

    println!("CallerContract contract id = {}", Address::from(*callercontract_cid.clone()));
    println!("TargetContract contract id = {}", Address::from(*targetcontract_id.clone()));


    //------------------------------------
    // get wallet0
    let secret0 = WALLET_FROM_FUEL_CORE;
    let wallet0 = WalletUnlocked::new_from_private_key(
        SecretKey::from_str(&secret0).unwrap(),
        Some(provider.clone())
    );

    let wal0addr = wallet0.address();
    let _wal0_identity = Identity::Address(wal0addr.into());

    println!("-----------");
    println!("wallet0 account:");
    println!("\t: 0x{}", wallet0.address());
    println!("\t: 0x{}\n", Address::from(wallet0.address()));

    //------------------------------------
    // CallerContract Instance:
    let callercontract_contract_instance = CallerContract::new(
        Bech32ContractId::from(callercontract_cid),
        wallet0.clone()
    );

    //------------------------------------
    // Setup low level call:

    let function_selector = fn_selector!(set0(u64));
    let call_data = calldata!(11u64);

    let _result = callercontract_contract_instance
        .methods()
        .call_low_level_call(
            targetcontract_id,
            Bytes(function_selector),
            Bytes(call_data),
            true,
        )
        .estimate_tx_dependencies(None)
        .await.unwrap()
        .call()
        .await;

    //------------------------------------
    // Check directly reading the TargetContract get_recover_address() method

    let target_contract_instance = TargetContract::new(
        Bech32ContractId::from(targetcontract_id),
        wallet0
    );

    let resultread = target_contract_instance
        .methods()
        .get0()
        .call()
        .await
        .unwrap()
        .value;

    println!("\n result.value = {:#?}", resultread);

    assert_eq!(resultread, 11u64);

}
// cargo test --package llcall_testing --test integration_tests -- test_set0_get0 --exact --show-output


///
/// # set1() and get1() -> This test fails.
///     PanicInstruction { reason: MemoryOverflow
///
///#ANCHOR - Tests the TargetContract methods set1() and get1() via low level call.
#[tokio::test]
async fn test_set1_get1() {
    println!("Tests TargetContract methods set1() and get1() via low level call:");

    let provider = match Provider::connect(RPC).await {
        Ok(p) => p,
        Err(error) => panic!("❌ Problem creating provider: {:#?}", error),
    };

    //------------------------------------
    // get CallerContract & TargetContract Contract ID from caller_cid.txt file.

    let callercontract_cid: ContractId = read_cid_from_file("caller_cid.txt".to_string()).unwrap();
    let targetcontract_id: ContractId = read_cid_from_file("target_cid.txt".to_string()).unwrap();
    println!("CallerContract contract id = {}", Address::from(*callercontract_cid.clone()));
    println!("TargetContract contract id = {}", Address::from(*targetcontract_id.clone()));

    //------------------------------------
    // get wallet0
    let secret0 = WALLET_FROM_FUEL_CORE;
    let wallet0 = WalletUnlocked::new_from_private_key(
        SecretKey::from_str(&secret0).unwrap(),
        Some(provider.clone())
    );
    let _wal0addr = wallet0.address();

    println!("-----------");
    println!("wallet0 account:");
    println!("\t: 0x{}", wallet0.address());
    println!("\t: 0x{}\n", Address::from(wallet0.address()));

    //------------------------------------
    // CallerContract Instance:
    let callercontract_contract_instance = CallerContract::new(
        Bech32ContractId::from(callercontract_cid),
        wallet0.clone()
    );

    //------------------------------------
    // Setup low level call:

    let hex_str = "0x0101010101010101010101010101010101010101010101010101010101010105";
    let addr_raw = Bits256::from_hex_str(hex_str).unwrap();

    let function_selector = fn_selector!(set1(Bits256));
    let call_data = calldata!(addr_raw);

    let _result = callercontract_contract_instance
        .methods()
        .call_low_level_call(
            targetcontract_id,
            Bytes(function_selector),
            Bytes(call_data),
            true,
        )
        .estimate_tx_dependencies(None)
        .await.unwrap()
        .call()
        .await;

    //FIXME - Errors out at the VM reverts tx with: "reason: PanicInstruction { reason: MemoryOverflow,..."

    //------------------------------------
    // Check directly reading the TargetContract get_recover_address() method

    let target_contract_instance = TargetContract::new(
        Bech32ContractId::from(targetcontract_id),
        wallet0
    );

    let resultread = target_contract_instance
        .methods()
        .get1()
        .call()
        .await
        .unwrap()
        .value;

    println!("\n result.value = {:#?}", resultread);

    //assert_eq!(resultread, 11u64);

}
// cargo test --package llcall_testing --test integration_tests -- test_set1_get1 --exact --show-output



///
/// # set2() and get2() -> This test fails.
///
///
///#ANCHOR - Tests the TargetContract methods set2() and get2() via low level call.
#[tokio::test]
async fn test_set2_get2() {
    println!("Tests TargetContract methods set2() and get2() via low level call:");

    let provider = match Provider::connect(RPC).await {
        Ok(p) => p,
        Err(error) => panic!("❌ Problem creating provider: {:#?}", error),
    };

    //------------------------------------
    // get CallerContract & TargetContract Contract ID from caller_cid.txt file.

    let callercontract_cid: ContractId = read_cid_from_file("caller_cid.txt".to_string()).unwrap();
    let targetcontract_id: ContractId = read_cid_from_file("target_cid.txt".to_string()).unwrap();
    println!("CallerContract contract id = {}", Address::from(*callercontract_cid.clone()));
    println!("TargetContract contract id = {}", Address::from(*targetcontract_id.clone()));

    //------------------------------------
    // get wallet0
    let secret0 = WALLET_FROM_FUEL_CORE;
    let wallet0 = WalletUnlocked::new_from_private_key(
        SecretKey::from_str(&secret0).unwrap(),
        Some(provider.clone())
    );
    let _wal0addr = wallet0.address();

    println!("-----------");
    println!("wallet0 account:");
    println!("\t: 0x{}", wallet0.address());
    println!("\t: 0x{}\n", Address::from(wallet0.address()));

    //------------------------------------
    // CallerContract Instance:
    let callercontract_contract_instance = CallerContract::new(
        Bech32ContractId::from(callercontract_cid),
        wallet0.clone()
    );

    //------------------------------------
    // Setup low level call:

    let function_selector = fn_selector!(set2(u64, u64));
    let call_data = calldata!(10u64, 11u64);

    let _result = callercontract_contract_instance
        .methods()
        .call_low_level_call(
            targetcontract_id,
            Bytes(function_selector),
            Bytes(call_data),
            true,
        )
        .estimate_tx_dependencies(None)
        .await.unwrap()
        .call()
        .await;


    //------------------------------------
    // Check directly reading the TargetContract get2() method directly.

    let target_contract_instance = TargetContract::new(
        Bech32ContractId::from(targetcontract_id),
        wallet0
    );

    let resultread = target_contract_instance
        .methods()
        .get2()
        .call()
        .await
        .unwrap()
        .value;

    println!("\n result.value = {:#?}", resultread);

    //FIXME - Calling the get2() method directly returns random garbage.
    // should return 10u64, 11u64
    // getting:
    // result.value = (
    //      8639018161287997909,
    //      6228131276584899535,
    // )

    assert_eq!(resultread.0, 10u64);
    assert_eq!(resultread.1, 11u64);

}
// cargo test --package llcall_testing --test integration_tests -- test_set2_get2 --exact --show-output


///
/// # set3() and get3() -> This test fails.
///
///
///#ANCHOR - Tests the TargetContract methods set3() and get3() via low level call
#[tokio::test]
async fn test_set3_get3() {
    println!("Tests TargetContract methods set3() and get3() by populating
    a DemoStruct and calling via low level call:");

    let provider = match Provider::connect(RPC).await {
        Ok(p) => p,
        Err(error) => panic!("❌ Problem creating provider: {:#?}", error),
    };

    //------------------------------------
    // get CallerContract & TargetContract Contract ID from caller_cid.txt file.

    let callercontract_cid: ContractId = read_cid_from_file("caller_cid.txt".to_string()).unwrap();
    let targetcontract_id: ContractId = read_cid_from_file("target_cid.txt".to_string()).unwrap();
    println!("CallerContract contract id = {}", Address::from(*callercontract_cid.clone()));
    println!("TargetContract contract id = {}", Address::from(*targetcontract_id.clone()));

    //------------------------------------
    // get wallet0
    let secret0 = WALLET_FROM_FUEL_CORE;
    let wallet0 = WalletUnlocked::new_from_private_key(
        SecretKey::from_str(&secret0).unwrap(),
        Some(provider.clone())
    );
    let _wal0addr = wallet0.address();

    println!("-----------");
    println!("wallet0 account:");
    println!("\t: 0x{}", wallet0.address());
    println!("\t: 0x{}\n", Address::from(wallet0.address()));

    //------------------------------------
    // CallerContract Instance:
    let callercontract_contract_instance = CallerContract::new(
        Bech32ContractId::from(callercontract_cid),
        wallet0.clone()
    );

    //------------------------------------
    // Setup low level call:

    let function_selector = fn_selector!(set3(DemoStruct));
    let call_data = calldata!(
        DemoStruct {
            a: true,
            b: [1, 2, 3],
            c: 22u64,
        }
    );

    let _result = callercontract_contract_instance
        .methods()
        .call_low_level_call(
            targetcontract_id,
            Bytes(function_selector),
            Bytes(call_data),
            true,
        )
        .estimate_tx_dependencies(None)
        .await.unwrap()
        .call()
        .await;


    //------------------------------------
    // Check directly reading the TargetContract get3() method directly.

    let target_contract_instance = TargetContract::new(
        Bech32ContractId::from(targetcontract_id),
        wallet0
    );

    let resultread = target_contract_instance
        .methods()
        .get3()
        .call()
        .await
        .unwrap()
        .value;

    println!("\n result.value = {:#?}", resultread);

    //FIXME - Calling the get3() method directly returns random garbage, except for the bool
    // should return:
    //  result.value = (
    //      1,      --> DemoStruct.b[0]
    //      3,      --> DemoStruct.b[2]
    //      22,     --> DemoStruct.c
    //      true,   --> DemoStruct.a
    //  )
    //
    // getting:
    //  result.value = (
    //      16879792781728292788,   --> DemoStruct.b[0] (wrong) --> garbage
    //      2777983893316890880,    --> DemoStruct.b[2] (wrong) --> garbage
    //      0,                      --> DemoStruct.c    (wrong) --> always 0
    //      true,                   --> DemoStruct.a    (correct)
    //  )

    assert_eq!(resultread.0, 1u64);
    assert_eq!(resultread.1, 3u64);
    assert_eq!(resultread.2, 22u64);
    assert_eq!(resultread.3, true);

}
// cargo test --package llcall_testing --test integration_tests -- test_set3_get3 --exact --show-output



//--------------------------------------------------------------------------------------
//SECTION - Sanity Check --> Call TestContract directly to prove the target contract itself works.

///
/// # set3() and get3() -> This test fails.
///
///
///#ANCHOR - Tests the TargetContract methods set3() and get3()
#[tokio::test]
async fn direct_call_set3_get3() {
    println!("Tests TargetContract methods set3() and get3() calling contract methods directly:");

    let provider = match Provider::connect(RPC).await {
        Ok(p) => p,
        Err(error) => panic!("❌ Problem creating provider: {:#?}", error),
    };

    //------------------------------------
    // get CallerContract & TargetContract Contract ID from caller_cid.txt file.

    let callercontract_cid: ContractId = read_cid_from_file("caller_cid.txt".to_string()).unwrap();
    let targetcontract_id: ContractId = read_cid_from_file("target_cid.txt".to_string()).unwrap();
    println!("CallerContract contract id = {}", Address::from(*callercontract_cid.clone()));
    println!("TargetContract contract id = {}", Address::from(*targetcontract_id.clone()));

    //------------------------------------
    // get wallet0
    let secret0 = WALLET_FROM_FUEL_CORE;
    let wallet0 = WalletUnlocked::new_from_private_key(
        SecretKey::from_str(&secret0).unwrap(),
        Some(provider.clone())
    );
    let _wal0addr = wallet0.address();

    println!("-----------");
    println!("wallet0 account:");
    println!("\t: 0x{}", wallet0.address());
    println!("\t: 0x{}\n", Address::from(wallet0.address()));

    //------------------------------------
    // Check directly write/read the TargetContract methods.

    let target_contract_instance = TargetContract::new(
        Bech32ContractId::from(targetcontract_id),
        wallet0
    );

    let ds = DemoStruct {
        a: true,
        b: [1, 2, 3],
        c: 22u64,
    };

    let _result = target_contract_instance
        .methods()
        .set3(ds)
        .call()
        .await
        .unwrap()
        .value;


    let resultread = target_contract_instance
        .methods()
        .get3()
        .call()
        .await
        .unwrap()
        .value;

    println!("\n result.value = {:#?}", resultread);

    //NOTE - Calling the get3() method directly returns expected arguements.
    // should return:
    //  result.value = (
    //      1,      --> DemoStruct.b[0]
    //      3,      --> DemoStruct.b[2]
    //      22,     --> DemoStruct.c
    //      true,   --> DemoStruct.a
    //  )
    //

    assert_eq!(resultread.0, 1u64);
    assert_eq!(resultread.1, 3u64);
    assert_eq!(resultread.2, 22u64);
    assert_eq!(resultread.3, true);

}
// cargo test --package llcall_testing --test integration_tests -- direct_call_set3_get3 --exact --show-output



//-------------------------------------------------
// helpers:

fn write_cid_to_file(filename: String, cid: ContractId) {
    let mut callerid_file = match File::create(filename.to_string()) {
        Ok(callerid_file) => callerid_file,
        Err(e) => {
            eprintln!("Error creating the file: {}", e);
            return;
        }
    };
    match callerid_file.write_all(cid.to_string().as_bytes()) {
        Ok(_) => println!("CallerContract hex id written to the file: {}", filename),
        Err(e) => eprintln!("Error writing to caller_cid.txt: {}", e),
    }
}

fn read_cid_from_file(filename: String) -> Result<ContractId> {

    let reader = io::BufReader::new(
        File::open(filename.to_string())?
    );
    let mut cid: ContractId = ContractId::default();
    if let Some(Ok(cidline)) = reader.lines().next() {
        //println!("Contract ID from first line: {}", cidline);
        cid = cidline.to_string()
            .parse()
            .expect("Invalid ID");
    } else {
        println!("Empty file or couldn't be read.");
    }
    Ok(cid)
}
