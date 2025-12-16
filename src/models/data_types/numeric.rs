use crate::{enums::data_feed_error::DataFeedError, traits::data_type::TDataType};

pub const VALID_NUMERIC_SIZES: [usize; 5] = [1, 2, 4, 8, 16];

pub struct Numeric<const SIZE: usize> {
    pub value: u128
}

impl<const SIZE: usize> Numeric<SIZE> {
    pub fn new(value: u128) -> Result<Self, DataFeedError> {
        let numeric = Numeric {
            value
        };

        numeric.validate()?;

        Ok(numeric)
    }
}

impl<const SIZE: usize> TDataType for Numeric<SIZE> {
    fn validate(&self) -> Result<(), DataFeedError> {
        match SIZE {
            1 => {
                if self.value < (u8::MIN as u128) || self.value > (u8::MAX as u128) {
                    return Err(DataFeedError::InvalidNumericSize(SIZE));
                }
            },
            2 => {
                if self.value < (u16::MIN as u128) || self.value > (u16::MAX as u128) {
                    return Err(DataFeedError::InvalidNumericSize(SIZE));
                }
            },
            4 => {
                if self.value < (u32::MIN as u128) || self.value > (u32::MAX as u128) {
                    return Err(DataFeedError::InvalidNumericSize(SIZE));
                }
            },
            8 => {
                if self.value < (u64::MIN as u128) || self.value > (u64::MAX as u128) {
                    return Err(DataFeedError::InvalidNumericSize(SIZE));
                }
            },
            16 => {
                
            }
            _ => {
                return Err(DataFeedError::Other("An invalid SIZE value was provided for Numeric type.".into()));
            }
        }

        Ok(())
    }
}