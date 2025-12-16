use crate::{enums::data_feed_error::DataFeedError, global_constants, models::data_types::{alpha::Alpha, numeric::Numeric}, traits::message::TMessage, util::buffer_utils};

const VALID_SIDE_VALUES: [char; 2] = ['B', 'S'];

const MESSAGE_TYPE_BYTE_OFFSET: usize = 0;
const NANOSECONDS_BYTE_OFFSET: usize = 1;
const ORDER_ID_BYTE_OFFSET: usize = 5;
const ORDER_BOOK_ID_BYTE_OFFSET: usize = 13;
const SIDE_BYTE_OFFSET: usize = 17;
const EXECUTED_QUANTITY_BYTE_OFFSET: usize = 18;
const MATCH_ID_BYTE_OFFSET: usize = 26;
const OWNER_PARTICIPANT_ID_BYTE_OFFSET: usize = 38;
const COUNTERPARTY_PARTICIPANT_ID_BYTE_OFFSET: usize = 45;

const MESSAGE_TYPE_LENGTH: usize = 1;
const NANOSECONDS_LENGTH: usize = 4;
const ORDER_ID_LENGTH: usize = 8;
const ORDER_BOOK_ID_LENGTH: usize = 4;
const SIDE_LENGTH: usize = 1;
const EXECUTED_QUANTITY_LENGTH: usize = 8;
const MATCH_ID_LENGTH: usize = 12;
const OWNER_PARTICIPANT_ID_LENGH: usize = 7;
const COUNTERPARTY_PARTICIPANT_ID_LENGTH: usize = 7;

pub struct OrderExecutedMessage {
    pub message_type: Alpha<MESSAGE_TYPE_LENGTH>,
    pub nanoseconds: Numeric<NANOSECONDS_LENGTH>,
    pub order_id: Numeric<ORDER_ID_LENGTH>,
    pub order_book_id: Numeric<ORDER_BOOK_ID_LENGTH>,
    pub side: Alpha<SIDE_LENGTH>,
    pub executed_quantity: Numeric<EXECUTED_QUANTITY_LENGTH>,
    pub match_id: Numeric<MATCH_ID_LENGTH>,
    pub owner_participant_id: Alpha<OWNER_PARTICIPANT_ID_LENGH>,
    pub counterparty_participant_id: Alpha<COUNTERPARTY_PARTICIPANT_ID_LENGTH>
}

impl OrderExecutedMessage {
    pub fn new(
        message_type: Alpha<MESSAGE_TYPE_LENGTH>,
        nanoseconds: Numeric<NANOSECONDS_LENGTH>,
        order_id: Numeric<ORDER_ID_LENGTH>,
        order_book_id: Numeric<ORDER_BOOK_ID_LENGTH>,
        side: Alpha<SIDE_LENGTH>,
        executed_quantity: Numeric<EXECUTED_QUANTITY_LENGTH>,
        match_id: Numeric<MATCH_ID_LENGTH>,
        owner_participant_id: Alpha<OWNER_PARTICIPANT_ID_LENGH>,
        counterparty_participant_id: Alpha<COUNTERPARTY_PARTICIPANT_ID_LENGTH>
    ) -> Result<Self, DataFeedError> {
        let message = OrderExecutedMessage {
            message_type,
            nanoseconds,
            order_id,
            order_book_id,
            side,
            executed_quantity,
            match_id,
            owner_participant_id,
            counterparty_participant_id
        };

        message.validate_fields()?;

        Ok(message)
    }
}

impl TMessage for OrderExecutedMessage {
    fn to_bin<const SIZE: usize>(&self) -> Result<[u8; SIZE], DataFeedError> {
        if SIZE != global_constants::ORDER_EXECUTED_MESSAGE_BYTE_COUNT {
            return Err(DataFeedError::InvalidMessageSize(global_constants::ORDER_EXECUTED_MESSAGE_BYTE_COUNT, SIZE));
        }

        let mut binary = [0u8; SIZE];

        buffer_utils::write_alpha(&mut binary, MESSAGE_TYPE_BYTE_OFFSET, &self.message_type.value);
        buffer_utils::write_num(&mut binary, NANOSECONDS_BYTE_OFFSET, &self.nanoseconds.value.to_be_bytes());
        buffer_utils::write_num(&mut binary, ORDER_ID_BYTE_OFFSET, &self.order_id.value.to_be_bytes());
        buffer_utils::write_num(&mut binary, ORDER_BOOK_ID_BYTE_OFFSET, &self.order_book_id.value.to_be_bytes());
        buffer_utils::write_alpha(&mut binary, SIDE_BYTE_OFFSET, &self.side.value);
        buffer_utils::write_num(&mut binary, EXECUTED_QUANTITY_BYTE_OFFSET, &self.executed_quantity.value.to_be_bytes());
        buffer_utils::write_num(&mut binary, MATCH_ID_BYTE_OFFSET, &self.match_id.value.to_be_bytes());
        buffer_utils::write_alpha(&mut binary, OWNER_PARTICIPANT_ID_BYTE_OFFSET, &self.owner_participant_id.value);
        buffer_utils::write_alpha(&mut binary, COUNTERPARTY_PARTICIPANT_ID_BYTE_OFFSET, &self.counterparty_participant_id.value);

        Ok(binary)
    }

    fn from_bin<const SIZE: usize>(binary_data: &[u8; SIZE]) -> Result<Self, DataFeedError> where Self: Sized {
        if SIZE != global_constants::ORDER_EXECUTED_MESSAGE_BYTE_COUNT {
            return Err(DataFeedError::InvalidMessageSize(global_constants::ORDER_EXECUTED_MESSAGE_BYTE_COUNT, SIZE));
        }

        Ok(
            OrderExecutedMessage::new(
                buffer_utils::read_alpha(binary_data, MESSAGE_TYPE_BYTE_OFFSET)?, 
                buffer_utils::read_numeric(binary_data, NANOSECONDS_BYTE_OFFSET)?, 
                buffer_utils::read_numeric(binary_data, ORDER_ID_BYTE_OFFSET)?, 
                buffer_utils::read_numeric(binary_data, ORDER_BOOK_ID_BYTE_OFFSET)?, 
                buffer_utils::read_alpha(binary_data, SIDE_BYTE_OFFSET)?, 
                buffer_utils::read_numeric(binary_data, EXECUTED_QUANTITY_BYTE_OFFSET)?, 
                buffer_utils::read_numeric(binary_data, MATCH_ID_BYTE_OFFSET)?, 
                buffer_utils::read_alpha(binary_data, OWNER_PARTICIPANT_ID_BYTE_OFFSET)?, 
                buffer_utils::read_alpha(binary_data, COUNTERPARTY_PARTICIPANT_ID_BYTE_OFFSET)?
            )?
        )
    }

    fn validate_fields(&self) -> Result<(), DataFeedError> {
        if self.message_type.value != ['E'] {
            return Err(DataFeedError::InvalidMessageType('E', self.message_type.value[0]));
        }

        if !VALID_SIDE_VALUES.contains(&self.side.value[0]) {
            return Err(DataFeedError::InvalidSideValue(self.side.value[0]));
        }

        Ok(())
    }
}