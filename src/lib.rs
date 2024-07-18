use bitcoin::ScriptBuf as RustBitcoinScriptBuf;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Script(pub RustBitcoinScriptBuf);

impl Script {
    pub fn new(raw_output_script: Vec<u8>) -> Self {
        let script: RustBitcoinScriptBuf = raw_output_script.into();
        Script(script)
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.to_bytes()
    }
}

impl From<RustBitcoinScriptBuf> for Script {
    fn from(script: RustBitcoinScriptBuf) -> Self {
        Script(script)
    }
}

uniffi::include_scaffolding!("bitcoin");
