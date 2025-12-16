use crate::{enums::data_feed_error::DataFeedError, global_constants, models::data_types::{alpha::Alpha, numeric::Numeric}, traits::message::TMessage, util::buffer_utils};

const MESSAGE_TYPE_BYTE_OFFSET: usize = 0;
const SECOND_BYTE_OFFSET: usize = 1;

const MESSAGE_TYPE_LENGTH: usize = 1;
const SECOND_LENGTH: usize = 4;

pub struct SecondsMessage {
    pub message_type: Alpha<MESSAGE_TYPE_LENGTH>,
    pub second: Numeric<SECOND_LENGTH>
}

impl SecondsMessage {
    pub fn new(
        message_type: Alpha<MESSAGE_TYPE_LENGTH>,
        second: Numeric<SECOND_LENGTH>
    ) -> Result<Self, DataFeedError> {
        let message = SecondsMessage {
            message_type,
            second
        };

        message.validate_fields()?;

        Ok(message)
    }
}

impl TMessage for SecondsMessage {
    fn to_bin<const SIZE: usize>(&self) -> Result<[u8; SIZE], DataFeedError> {
        if SIZE != global_constants::SECONDS_MESSAGE_BYTE_COUNT {
            return Err(DataFeedError::InvalidMessageSize(global_constants::SECONDS_MESSAGE_BYTE_COUNT, SIZE));
        }

        let mut binary = [0u8; SIZE];

        buffer_utils::write_alpha(&mut binary, MESSAGE_TYPE_BYTE_OFFSET, &self.message_type.value);
        buffer_utils::write_num(&mut binary, SECOND_BYTE_OFFSET, &self.second.value.to_be_bytes());

        Ok(binary)
    }

    fn from_bin<const SIZE: usize>(binary_data: &[u8; SIZE]) -> Result<Self, DataFeedError> where Self: Sized {
        if SIZE != global_constants::SECONDS_MESSAGE_BYTE_COUNT {
            return Err(DataFeedError::InvalidMessageSize(global_constants::SECONDS_MESSAGE_BYTE_COUNT, SIZE));
        }

        Ok(
            SecondsMessage::new(
                buffer_utils::read_alpha(binary_data, MESSAGE_TYPE_BYTE_OFFSET)?, 
                buffer_utils::read_numeric(binary_data, SECOND_BYTE_OFFSET)?
            )?
        )
    }
    
    fn validate_fields(&self) -> Result<(), DataFeedError> {
        if self.message_type.value != ['T'] {
            return Err(DataFeedError::InvalidMessageType('T', self.message_type.value[0]));
        }

        Ok(())
    }
}