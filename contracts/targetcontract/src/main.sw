contract;

use std::constants::{
    BASE_ASSET_ID,
    ZERO_B256
};
use std::storage::storage_api::{read, write};
use std::bytes::Bytes;
use std::convert::TryFrom;
use std::option::Option::{self, *};



abi TargetContract {

    #[storage(write)]
    fn set0(a: u64);
    #[storage(read)]
    fn get0() -> u64;

    #[storage(write)]
    fn set1(a: b256);
    #[storage(read)]
    fn get1() -> b256;

    #[storage(write)]
    fn set2(a: u64, b: u64);
    #[storage(read)]
    fn get2() -> (u64, u64);

    #[storage(write)]
    fn set3(x: DemoStruct);
    #[storage(read)]
    fn get3() -> (u64, u64, u64, bool);


}

const STORAGE_KEY1: b256 = 0x0000000000000000000000000000000000000000000000000000000000000001;
const STORAGE_KEY2: b256 = 0x0000000000000000000000000000000000000000000000000000000000000002;


storage {
    var0: u64 = 0,
    var1: u64 = 0,
    var2: u64 = 0,
    var3: u64 = 0,
    var4: bool = false,
}

pub struct DemoStruct {
    a: bool,
    b: [u64; 3],
    c: u64,
}

impl TargetContract for Contract {

    #[storage(write)]
    fn set0(a: u64) {
        storage.var0.write(a);
    }
    #[storage(read)]
    fn get0() -> u64 {
        storage.var0.read()
    }


    // Attempt to get a b256 from calldata:
    //NOTE -  Errors out at the VM reverts tx with: "reason: PanicInstruction { reason: MemoryOverflow,..."
    //        when called via low level call.
    #[storage(write)]
    fn set1(a: b256) {
        let raw_address = a;
        //let raw_address: b256 = 0xddec0e7e6a9a4a4e3e57d08d080d71a299c628a46bc609aab4627695679421ca; // uncoment t
        write(STORAGE_KEY1, 0, raw_address);
    }
    #[storage(read)]
    fn get1() -> b256 {
        let value: Option<b256> = read::<b256>(STORAGE_KEY1, 0);
        value.unwrap_or(ZERO_B256)
    }

    // Attempt to two u64's:
    //NOTE - get2() does not return the values set by set2()
    //     - get2() when called directly, reurns garbage values if set2() is
    //       called via a low level call.
    #[storage(write)]
    fn set2(a: u64, b: u64) {
        //assert(a == 1u64);
        write(STORAGE_KEY2, 0, a);
        write(STORAGE_KEY2, 1, b);
    }
    #[storage(read)]
    fn get2() -> (u64, u64) {
        let a: Option<u64> = read::<u64>(STORAGE_KEY2, 0);
        let b: Option<u64> = read::<u64>(STORAGE_KEY2, 1);
        (a.unwrap_or(0), b.unwrap_or(0))
    }

    // Attempt to read DemoStruct elements:
    //NOTE - get3() does not return the values set by set3()
    //
    #[storage(write)]
    fn set3(x: DemoStruct) {
        storage.var1.write(x.b[0]);
        storage.var2.write(x.b[2]);
        storage.var3.write(x.c);
        storage.var4.write(x.a);
    }
    #[storage(read)]
    fn get3() -> (u64, u64, u64, bool) {
        (storage.var1.read(), storage.var2.read(), storage.var3.read(), storage.var4.read())
    }

}
