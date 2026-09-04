use std::io::Read;

use crate::error::{RecoveryError, RecoveryResult};

use super::types::ByteRange;

/// Describes a read-only source of forensic evidence.
pub trait EvidenceSource {
    /// Returns the total size of the evidence source in bytes.
    fn size(&self) -> u64;

    /// Reads a range after validating that it fits within the source.
    fn read_range(&mut self, range: ByteRange) -> RecoveryResult<Vec<u8>>;
}

/// Reads a range from a standard reader.
pub fn read_from<R: Read>(reader: &mut R, range: ByteRange) -> RecoveryResult<Vec<u8>> {
    let length = usize::try_from(range.length()).map_err(|_| {
        RecoveryError::InvalidArgument("requested range is too large for this platform".to_string())
    })?;

    let mut buffer = vec![0_u8; length];

    reader
        .read_exact(&mut buffer)
        .map_err(|error| RecoveryError::Io(error.to_string()))?;

    Ok(buffer)
}
