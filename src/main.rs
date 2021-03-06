use anyhow::anyhow;
use anyhow::Result;
use bee_message::{output::Output, output::OutputId};
use serde::{Deserialize, Serialize};

use std::collections::HashMap;
use std::{fs::File, io::BufWriter, path::Path};

mod snapshot;
use snapshot::get_snapshot_data;

const SNAPSHOT_PATH: &str = "full_snapshot.bin";
const OUTPUT_PATH: &str = "snapshot.json";
const BECH_32_HRP: &str = "iota";

// Data of all addresses and the treasury for a certain ledger_index
#[derive(Clone, Serialize, Deserialize)]
pub struct SnapshotData {
    // The milestone index
    #[serde(rename = "ledgerIndex")]
    ledger_index: u32,
    // All addresses bech32 encoded with their balance and output ids
    addresses: HashMap<String, AddressData>,
    // The remaining amount in the treasury (legacy network)
    #[serde(rename = "treasuryOutputAmount")]
    treasury_output_amount: u64,
}

/// Data for an address
#[derive(Clone, Serialize, Deserialize)]
pub struct AddressData {
    // The total balance of the address
    pub balance: u64,
    // The output ids of the outputs for this address
    pub output_ids: Vec<OutputId>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let snapshot_data = read_snapshot_addresses_data(SNAPSHOT_PATH)?;
    write_to_file(
        format!("{}{}", snapshot_data.ledger_index, OUTPUT_PATH),
        snapshot_data,
    )?;
    Ok(())
}

// Read snapshot file and write addresses with their balance to a file in json format
fn read_snapshot_addresses_data(snapshot_path: &str) -> Result<SnapshotData> {
    let mut addresses = HashMap::new();
    let (outputs, treasury_output_amount, ledger_index) = get_snapshot_data(snapshot_path)?;
    for output_data in outputs.into_iter() {
        let (amount, address) = get_output_amount_and_address(&output_data.output)?;
        addresses
            .entry(address)
            .and_modify(|e: &mut AddressData| {
                e.balance += amount;
                e.output_ids.push(output_data.output_id);
            })
            .or_insert_with(|| AddressData {
                balance: amount,
                output_ids: vec![output_data.output_id],
            });
    }
    Ok(SnapshotData {
        ledger_index,
        addresses,
        treasury_output_amount,
    })
}

/// Get output amount and address from an Output
pub fn get_output_amount_and_address(output: &Output) -> Result<(u64, String)> {
    let (amount, address) = match output {
        Output::Treasury(_) => return Err(anyhow!("Treasury output is not allowed")),
        Output::SignatureLockedSingle(ref r) => (r.amount(), r.address()),
        Output::SignatureLockedDustAllowance(ref r) => (r.amount(), r.address()),
    };
    Ok((amount, address.to_bech32(BECH_32_HRP)))
}

/// Function to write address snapshot data to a file
pub fn write_to_file<P: AsRef<Path>>(path: P, snapshot_data: SnapshotData) -> Result<()> {
    let jsonvalue = serde_json::to_value(&snapshot_data)?;
    let file = File::create(path)?;
    let bw = BufWriter::new(file);
    serde_json::to_writer_pretty(bw, &jsonvalue)?;
    Ok(())
}
