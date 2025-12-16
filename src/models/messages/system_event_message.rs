use crate::{enums::data_feed_error::DataFeedError, global_constants, models::data_types::{alpha::Alpha, numeric::Numeric}, traits::message::TMessage, util::buffer_utils};

const VALID_EVENT_CODE_VALUES: [char; 2] = ['O', 'C'];

const MESSAGE_TYPE_BYTE_OFFSET: usize = 0;
const NANOSECONDS_BYTE_OFFSET: usize = 1;
const EVENT_CODE_BYTE_OFFSET: usize = 5;

const MESSAGE_TYPE_LENGTH: usize = 1;
const NANOSECONDS_LENGTH: usize = 4;
const EVENT_CODE_LENGTH: usize = 1;

pub struct SystemEventMessage {
    pub message_type: Alpha<MESSAGE_TYPE_LENGTH>,
    pub nanoseconds: Numeric<NANOSECONDS_LENGTH>,
    pub event_code: Alpha<EVENT_CODE_LENGTH>
}

impl SystemEventMessage {
    pub fn new(
        message_type: Alpha<MESSAGE_TYPE_LENGTH>,
        nanoseconds: Numeric<NANOSECONDS_LENGTH>,
        event_code: Alpha<EVENT_CODE_LENGTH>
    ) -> Result<Self, DataFeedError> {
        let message = SystemEventMessage {
            message_type,
            nanoseconds,
            event_code
        };

        message.validate_fields()?;

        Ok(message)
    }
}

impl TMessage for SystemEventMessage {
    fn to_bin<const SIZE: usize>(&self) -> Result<[u8; SIZE], DataFeedError> {
        if SIZE != global_constants::SYSTEM_EVENT_MESSAGE_BYTE_COUNT {
            return Err(DataFeedError::InvalidMessageSize(global_constants::SYSTEM_EVENT_MESSAGE_BYTE_COUNT, SIZE));
        }

        let mut binary = [0u8; SIZE];

        buffer_utils::write_alpha(&mut binary, MESSAGE_TYPE_BYTE_OFFSET, &self.message_type.value);
        buffer_utils::write_num(&mut binary, NANOSECONDS_BYTE_OFFSET, &self.nanoseconds.value.to_be_bytes());
        buffer_utils::write_alpha(&mut binary, EVENT_CODE_BYTE_OFFSET, &self.event_code.value);

        Ok(binary)
    }

    fn from_bin<const SIZE: usize>(binary_data: &[u8; SIZE]) -> Result<Self, DataFeedError> where Self: Sized {
        if SIZE != global_constants::SYSTEM_EVENT_MESSAGE_BYTE_COUNT {
            return Err(DataFeedError::InvalidMessageSize(global_constants::SYSTEM_EVENT_MESSAGE_BYTE_COUNT, SIZE));
        }

        Ok(
            SystemEventMessage::new(
                buffer_utils::read_alpha(binary_data, MESSAGE_TYPE_BYTE_OFFSET)?, 
                buffer_utils::read_numeric(binary_data, NANOSECONDS_BYTE_OFFSET)?, 
                buffer_utils::read_alpha(binary_data, EVENT_CODE_BYTE_OFFSET)?
            )?
        )
    }
    
    fn validate_fields(&self) -> Result<(), DataFeedError> {
        if self.message_type.value != ['S'] {
            return Err(DataFeedError::InvalidMessageType('S', self.message_type.value[0]));
        }

        if !VALID_EVENT_CODE_VALUES.contains(&self.event_code.value[0]) {
            return Err(DataFeedError::InvalidEventCodeValue(self.event_code.value[0]));
        }

        Ok(())
    }
}