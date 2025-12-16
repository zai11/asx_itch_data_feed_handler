use crate::{enums::data_feed_error::DataFeedError, global_constants, models::data_types::{alpha::Alpha, numeric::Numeric, price::Price}, traits::message::TMessage, util::buffer_utils};

const MESSAGE_TYPE_BYTE_OFFSET: usize = 0;
const NANOSECONDS_BYTE_OFFSET: usize = 1;
const ORDER_BOOK_ID_BYTE_OFFSET: usize = 5;
const TICK_SIZE_BYTE_OFFSET: usize = 9;
const PRICE_FROM_BYTE_OFFSET: usize = 17;
const PRICE_TO_BYTE_OFFSET: usize = 21;

const MESSAGE_TYPE_LENGTH: usize = 1;
const NANOSECONDS_LENGTH: usize = 4;
const ORDER_BOOK_ID_LENGTH: usize = 4;
const TICK_SIZE_LENGTH: usize = 8;

pub struct TickSizeTableEntryMessage {
    pub message_type: Alpha<MESSAGE_TYPE_LENGTH>,
    pub nanoseconds: Numeric<NANOSECONDS_LENGTH>,
    pub order_book_id: Numeric<ORDER_BOOK_ID_LENGTH>,
    pub tick_size: Numeric<TICK_SIZE_LENGTH>,
    pub price_from: Price,
    pub price_to: Price
}

impl TickSizeTableEntryMessage {
    pub fn new(
        message_type: Alpha<MESSAGE_TYPE_LENGTH>,
        nanoseconds: Numeric<NANOSECONDS_LENGTH>,
        order_book_id: Numeric<ORDER_BOOK_ID_LENGTH>,
        tick_size: Numeric<TICK_SIZE_LENGTH>,
        price_from: Price,
        price_to: Price
    ) -> Result<Self, DataFeedError> {
        let message = TickSizeTableEntryMessage {
            message_type,
            nanoseconds,
            order_book_id,
            tick_size,
            price_from,
            price_to
        };

        message.validate_fields()?;

        Ok(message)
    }
}

impl TMessage for TickSizeTableEntryMessage {
    fn to_bin<const SIZE: usize>(&self) -> Result<[u8; SIZE], DataFeedError> {
        if SIZE != global_constants::TICK_SIZE_TABLE_ENTRY_MESSAGE_BYTE_COUNT {
            return Err(DataFeedError::InvalidMessageSize(global_constants::TICK_SIZE_TABLE_ENTRY_MESSAGE_BYTE_COUNT, SIZE));
        }

        let mut binary = [0u8; SIZE];

        buffer_utils::write_alpha(&mut binary, MESSAGE_TYPE_BYTE_OFFSET, &self.message_type.value);
        buffer_utils::write_num(&mut binary, NANOSECONDS_BYTE_OFFSET, &self.nanoseconds.value.to_be_bytes());
        buffer_utils::write_num(&mut binary, ORDER_BOOK_ID_BYTE_OFFSET, &self.order_book_id.value.to_be_bytes());
        buffer_utils::write_num(&mut binary, TICK_SIZE_BYTE_OFFSET, &self.tick_size.value.to_be_bytes());
        buffer_utils::write_num(&mut binary, PRICE_FROM_BYTE_OFFSET, &self.price_from.value.to_be_bytes());
        buffer_utils::write_num(&mut binary, PRICE_TO_BYTE_OFFSET, &self.price_to.value.to_be_bytes());

        Ok(binary)
    }

    fn from_bin<const SIZE: usize>(binary_data: &[u8; SIZE]) -> Result<Self, DataFeedError> where Self: Sized {
        if SIZE != global_constants::TICK_SIZE_TABLE_ENTRY_MESSAGE_BYTE_COUNT {
            return Err(DataFeedError::InvalidMessageSize(global_constants::TICK_SIZE_TABLE_ENTRY_MESSAGE_BYTE_COUNT, SIZE));
        }

        Ok(
            TickSizeTableEntryMessage::new(
                buffer_utils::read_alpha(binary_data, MESSAGE_TYPE_BYTE_OFFSET)?, 
                buffer_utils::read_numeric(binary_data, NANOSECONDS_BYTE_OFFSET)?, 
                buffer_utils::read_numeric(binary_data, ORDER_BOOK_ID_BYTE_OFFSET)?, 
                buffer_utils::read_numeric(binary_data, TICK_SIZE_BYTE_OFFSET)?, 
                buffer_utils::read_price(binary_data, PRICE_FROM_BYTE_OFFSET), 
                buffer_utils::read_price(binary_data, PRICE_TO_BYTE_OFFSET)
            )?
        )
    }
    
    fn validate_fields(&self) -> Result<(), DataFeedError> {
        if self.message_type.value != ['L'] {
            return Err(DataFeedError::InvalidMessageType('L', self.message_type.value[0]));
        }

        Ok(())
    }
}