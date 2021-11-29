use anyhow::anyhow;
use anyhow::Result;
use bee_common::packable::{Packable, Read};
use bee_ledger::types::snapshot::{FullSnapshotHeader, SnapshotHeader, SnapshotKind};
use bee_message::{
    output::{Output, OutputId},
    MessageId,
};
use bee_tangle::solid_entry_point::SolidEntryPoint;
use std::{fs::OpenOptions, io::BufReader, path::Path};

#[derive(Debug, Clone)]
pub struct OutputData {
    pub output_id: OutputId,
    pub output: Output,
    pub message_id: MessageId,
}

/// Get outputs, treasury_output_amount and ledger_index from a snapshot
pub fn get_snapshot_data(path: &str) -> Result<(Vec<OutputData>, u64, u32)> {
    let mut reader = BufReader::new(OpenOptions::new().read(true).open(Path::new(&path))?);

    let header = SnapshotHeader::unpack(&mut reader)?;

    println!("Network ID:\t\t\t{}", header.network_id());
    println!("Ledger index:\t\t\t{}", *header.ledger_index());

    if let SnapshotKind::Full = header.kind() {
        let full_header = FullSnapshotHeader::unpack(&mut reader)?;

        // Read solid entry points so we can access the outputs that come after them
        for _ in 0..full_header.sep_count() {
            SolidEntryPoint::unpack(&mut reader)?;
        }
        let outputs = import_outputs(&mut reader, full_header.output_count())?;
        Ok((
            outputs,
            full_header.treasury_output_amount(),
            *header.ledger_index(),
        ))
    } else {
        Err(anyhow!("Full snapshot required"))
    }
}

// Read OutputData from the reader
fn import_outputs<R: Read>(reader: &mut R, output_count: u64) -> Result<Vec<OutputData>> {
    let mut outputs = Vec::new();
    for _ in 0..output_count {
        let message_id = MessageId::unpack(reader)?;
        let output_id = OutputId::unpack(reader)?;
        let output = Output::unpack(reader)?;

        outputs.push(OutputData {
            message_id,
            output,
            output_id,
        })
    }
    Ok(outputs)
}
