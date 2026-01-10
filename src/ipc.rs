use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum ServiceCommand
{
        WriteRegDword
        {
                root: RegRoot,
                subkey: String,
                value: String,
                data: u32,
        },
        WriteRegString
        {
                root: RegRoot,
                subkey: String,
                value: String,
                data: String,
        },
        DeleteRegValue
        {
                root: RegRoot,
                subkey: String,
                value: String,
        },
        DeleteRegKey
        {
                root: RegRoot,
                subkey: String,
        },
        Ping,
        Stop,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum RegRoot
{
        HKLM,
        HKCU,
        HKCR,
        HKU,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ServiceResponse
{
        Success,
        Error(String),
}
