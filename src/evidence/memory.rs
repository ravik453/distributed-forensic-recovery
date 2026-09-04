use super::source::EvidenceSource;
use super::types::ByteRange;
use crate::error::RecoveryResult;

/// Read-only evidence source backed by memory.
///
/// This is intended for tests and small deterministic fixtures.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryEvidence {
    bytes: Vec<u8>,
}

impl MemoryEvidence {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }
}

impl EvidenceSource for MemoryEvidence {
    fn size(&self) -> u64 {
        self.bytes.len() as u64
    }

    fn read_range(&mut self, range: ByteRange) -> RecoveryResult<Vec<u8>> {
        range.validate_against(self.size())?;

        let start = usize::try_from(range.offset()).map_err(|_| {
            crate::error::RecoveryError::InvalidArgument(
                "range offset is too large for this platform".to_string(),
            )
        })?;

        let end = usize::try_from(range.end()?).map_err(|_| {
            crate::error::RecoveryError::InvalidArgument(
                "range end is too large for this platform".to_string(),
            )
        })?;

        Ok(self.bytes[start..end].to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_a_valid_range() {
        let mut source = MemoryEvidence::new(vec![10, 20, 30, 40, 50]);
        let range = ByteRange::new(1, 3).expect("range should be valid");

        assert_eq!(
            source.read_range(range).expect("read should succeed"),
            vec![20, 30, 40]
        );
    }

    #[test]
    fn rejects_a_range_beyond_source() {
        let mut source = MemoryEvidence::new(vec![10, 20, 30]);
        let range = ByteRange::new(2, 2).expect("range should be valid");

        assert!(source.read_range(range).is_err());
    }
}
