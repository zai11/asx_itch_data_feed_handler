use crate::{enums::data_feed_error::DataFeedError, global_constants, models::data_types::{alpha::Alpha, numeric::Numeric}, traits::message::TMessage, util::buffer_utils};

const MESSAGE_TYPE_BYTE_OFFSET: usize = 0;
const NANOSECONDS_BYTE_OFFSET: usize = 1;
const ORDER_BOOK_ID_BYTE_OFFSET: usize = 5;
const STATE_NAME_BYTE_OFFSET: usize = 9;

const MESSAGE_TYPE_LENGTH: usize = 1;
const NANOSECONDS_LENGTH: usize = 4;
const ORDER_BOOK_ID_LENGTH: usize = 4;
const STATE_NAME_LENGTH: usize = 20;

pub struct OrderBookStateMessage {
    pub message_type: Alpha<MESSAGE_TYPE_LENGTH>,
    pub nanoseconds: Numeric<NANOSECONDS_LENGTH>,
    pub order_book_id: Numeric<ORDER_BOOK_ID_LENGTH>,
    pub state_name: Alpha<STATE_NAME_LENGTH>
}

impl OrderBookStateMessage {
    pub fn new(
        message_type: Alpha<MESSAGE_TYPE_LENGTH>,
        nanoseconds: Numeric<NANOSECONDS_LENGTH>,
        order_book_id: Numeric<ORDER_BOOK_ID_LENGTH>,
        state_name: Alpha<STATE_NAME_LENGTH>
    ) -> Result<Self, DataFeedError> {
        let message = OrderBookStateMessage {
            message_type,
            nanoseconds,
            order_book_id,
            state_name
        };

        message.validate_fields()?;

        Ok(message)
    }
}

impl TMessage for OrderBookStateMessage {
    fn to_bin<const SIZE: usize>(&self) -> Result<[u8; SIZE], DataFeedError> {
        if SIZE != global_constants::ORDER_BOOK_STATE_MESSAGE_BYTE_COUNT {
            return Err(DataFeedError::InvalidMessageSize(global_constants::ORDER_BOOK_STATE_MESSAGE_BYTE_COUNT, SIZE));
        }

        let mut binary = [0u8; SIZE];

        buffer_utils::write_alpha(&mut binary, MESSAGE_TYPE_BYTE_OFFSET, &self.message_type.value);
        buffer_utils::write_num(&mut binary, NANOSECONDS_BYTE_OFFSET, &self.nanoseconds.value.to_be_bytes());
        buffer_utils::write_num(&mut binary, ORDER_BOOK_ID_BYTE_OFFSET, &self.order_book_id.value.to_be_bytes());
        buffer_utils::write_alpha(&mut binary, STATE_NAME_BYTE_OFFSET, &self.state_name.value);

        Ok(binary)
    }

    fn from_bin<const SIZE: usize>(binary_data: &[u8; SIZE]) -> Result<Self, DataFeedError> where Self: Sized {
        if SIZE != global_constants::ORDER_BOOK_STATE_MESSAGE_BYTE_COUNT {
            return Err(DataFeedError::InvalidMessageSize(global_constants::ORDER_BOOK_STATE_MESSAGE_BYTE_COUNT, SIZE));
        }

        Ok(
            OrderBookStateMessage::new(
                buffer_utils::read_alpha(binary_data, MESSAGE_TYPE_BYTE_OFFSET)?, 
                buffer_utils::read_numeric(binary_data, NANOSECONDS_BYTE_OFFSET)?, 
                buffer_utils::read_numeric(binary_data, ORDER_BOOK_ID_BYTE_OFFSET)?, 
                buffer_utils::read_alpha(binary_data, STATE_NAME_BYTE_OFFSET)?
            )?
        )
    }
    
    fn validate_fields(&self) -> Result<(), DataFeedError> {
        if self.message_type.value != ['O'] {
            return Err(DataFeedError::InvalidMessageType('O', self.message_type.value[0]));
        }

        Ok(())
    }
}