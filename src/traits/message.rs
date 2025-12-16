use crate::enums::data_feed_error::DataFeedError;

pub trait TMessage {
    fn to_bin<const SIZE: usize>(&self) -> Result<[u8; SIZE], DataFeedError>;
    fn from_bin<const SIZE: usize>(binary_data: &[u8; SIZE]) -> Result<Self, DataFeedError> where Self: Sized;
    fn validate_fields(&self) -> Result<(), DataFeedError>;
}