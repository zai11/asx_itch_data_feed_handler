use crate::enums::data_feed_error::DataFeedError;

pub trait TDataType {
    fn validate(&self) -> Result<(), DataFeedError>;
}