use crate::{enums::data_feed_error::DataFeedError, traits::data_type::TDataType};

pub struct Alpha<const SIZE: usize> {
    pub value: [char; SIZE]
}

impl<const SIZE: usize> Alpha<SIZE> {
    pub fn new(value: [char; SIZE]) -> Result<Self, DataFeedError> {
        let alpha = Alpha {
            value
        };

        alpha.validate()?;

        Ok(alpha)
    }
}

impl<const SIZE: usize> TDataType for Alpha<SIZE> {
    fn validate(&self) -> Result<(), DataFeedError> {
        for c in self.value {
            if (c as u16) > 0xFF {
                return Err(DataFeedError::NonISO88591Value(c));
            }
        }
        
        Ok(())
    }
}