#![feature(abi_efiapi)]
#[macro_use]
extern crate uefi_derive;
use guid;
#[derive(Protocol)]
#[guid = "72631e54-78a4-11d0-bcf7-00aa00b7b32a"]
pub struct A;
