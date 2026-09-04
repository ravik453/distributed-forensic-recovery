use crate::error::{RecoveryError, RecoveryResult};

/// A validated byte range within an evidence source.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ByteRange {
    offset: u64,
    length: u64,
}

impl ByteRange {
    /// Creates a byte range after validating that its length is non-zero.
    pub fn new(offset: u64, length: u64) -> RecoveryResult<Self> {
        if length == 0 {
            return Err(RecoveryError::InvalidRange { offset, length });
        }

        Ok(Self { offset, length })
    }

    pub const fn offset(self) -> u64 {
        self.offset
    }

    pub const fn length(self) -> u64 {
        self.length
    }

    /// Returns the exclusive end offset, if it can be represented.
    pub fn end(self) -> RecoveryResult<u64> {
        self.offset
            .checked_add(self.length)
            .ok_or_else(|| RecoveryError::InvalidRange {
                offset: self.offset,
                length: self.length,
            })
    }

    /// Validates that this range fits within an evidence source.
    pub fn validate_against(self, source_size: u64) -> RecoveryResult<()> {
        let end = self.end()?;

        if end > source_size {
            return Err(RecoveryError::OutOfBounds {
                offset: self.offset,
                length: self.length,
                source_size,
            });
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_valid_range() {
        let range = ByteRange::new(100, 50).expect("range should be valid");

        assert_eq!(range.offset(), 100);
        assert_eq!(range.length(), 50);
        assert_eq!(range.end().expect("end should be valid"), 150);
    }

    #[test]
    fn rejects_zero_length_range() {
        let result = ByteRange::new(100, 0);

        assert_eq!(
            result,
            Err(RecoveryError::InvalidRange {
                offset: 100,
                length: 0,
            })
        );
    }

    #[test]
    fn rejects_overflowing_range() {
        let result = ByteRange::new(u64::MAX, 1)
            .expect("construction itself is valid")
            .end();

        assert_eq!(
            result,
            Err(RecoveryError::InvalidRange {
                offset: u64::MAX,
                length: 1,
            })
        );
    }

    #[test]
    fn rejects_range_beyond_source() {
        let range = ByteRange::new(90, 20).expect("range should be valid");

        assert_eq!(
            range.validate_against(100),
            Err(RecoveryError::OutOfBounds {
                offset: 90,
                length: 20,
                source_size: 100,
            })
        );
    }
}
