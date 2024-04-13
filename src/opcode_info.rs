use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub struct FlagInformation {
    pub z: String,
    pub n: String,
    pub h: String,
    pub c: String,
}

#[derive(Deserialize, Debug)]
pub struct OperandInformation {
    pub name: String,
    pub immediate: bool,
}

#[derive(Deserialize, Debug)]
pub struct Opcode {
    pub operands: Vec<OperandInformation>,
    pub mnemonic: String,
    pub bytes: u8,
    pub immediate: bool,
    pub cycles: Vec<u8>,
    pub flags: FlagInformation,
}

#[derive(Deserialize, Debug)]
pub struct OpcodeInfo {
    pub unprefixed: HashMap<String, Opcode>,
    pub cbprefixed: HashMap<String, Opcode>,
}
