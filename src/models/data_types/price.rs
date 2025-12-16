use crate::{enums::data_feed_error::DataFeedError, traits::data_type::TDataType};

pub struct Price {
    pub value: i32
}

impl Price {
    pub fn new(value: i32) -> Result<Self, DataFeedError> {
        let price = Price {
            value
        };

        price.validate()?;

        Ok(price)
    }
}

impl TDataType for Price {
    fn validate(&self) -> Result<(), DataFeedError> {
        Ok(())
    }
}