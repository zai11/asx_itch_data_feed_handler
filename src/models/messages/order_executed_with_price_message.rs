use crate::{enums::data_feed_error::DataFeedError, global_constants, models::data_types::{alpha::Alpha, numeric::Numeric, price::Price}, traits::message::TMessage, util::buffer_utils};

const VALID_SIDE_VALUES: [char; 2] = ['B', 'S'];
const VALID_OCCURRED_AT_CROSS_VALUES: [char; 2] = ['N', 'Y'];
const VALID_PRINTABLE_VALUES: [char; 2] = ['N', 'Y'];

const MESSAGE_TYPE_BYTE_OFFSET: usize = 0;
const NANOSECONDS_BYTE_OFFSET: usize = 1;
const ORDER_ID_BYTE_OFFSET: usize = 5;
const ORDER_BOOK_ID_BYTE_OFFSET: usize = 13;
const SIDE_BYTE_OFFSET: usize = 17;
const EXECUTED_QUANTITY_BYTE_OFFSET: usize = 18;
const MATCH_ID_BYTE_OFFSET: usize = 26;
const OWNER_PARTICIPANT_ID_BYTE_OFFSET: usize = 38;
const COUNTERPARTY_PARTICIPANT_ID_BYTE_OFFSET: usize = 45;
const TRADE_PRICE_BYTE_OFFSET: usize = 52;
const OCCURRED_AT_CROSS_BYTE_OFFSET: usize = 56;
const PRINTABLE_BYTE_OFFSET: usize = 57;

const MESSAGE_TYPE_LENGTH: usize = 1;
const NANOSECONDS_LENGTH: usize = 4;
const ORDER_ID_LENGTH: usize = 8;
const ORDER_BOOK_ID_LENGTH: usize = 4;
const SIDE_LENGTH: usize = 1;
const EXECUTED_QUANTITY_LENGTH: usize = 8;
const MATCH_ID_LENGTH: usize = 12;
const OWNER_PARTICIPANT_ID_LENGTH: usize = 7;
const COUNTERPARTY_PARTICIPANT_ID_LENGTH: usize = 7;
const OCCURRED_AT_CROSS_LENGTH: usize = 1;
const PRINTABLE_LENGTH: usize = 1;

pub struct OrderExecutedWithPriceMessage {
    pub message_type: Alpha<MESSAGE_TYPE_LENGTH>,
    pub nanoseconds: Numeric<NANOSECONDS_LENGTH>,
    pub order_id: Numeric<ORDER_ID_LENGTH>,
    pub order_book_id: Numeric<ORDER_BOOK_ID_LENGTH>,
    pub side: Alpha<SIDE_LENGTH>,
    pub executed_quantity: Numeric<EXECUTED_QUANTITY_LENGTH>,
    pub match_id: Numeric<MATCH_ID_LENGTH>,
    pub owner_participant_id: Alpha<OWNER_PARTICIPANT_ID_LENGTH>,
    pub counterparty_participant_id: Alpha<COUNTERPARTY_PARTICIPANT_ID_LENGTH>,
    pub trade_price: Price,
    pub occurred_at_cross: Alpha<OCCURRED_AT_CROSS_LENGTH>,
    pub printable: Alpha<PRINTABLE_LENGTH>
}

impl OrderExecutedWithPriceMessage {
    pub fn new(
        message_type: Alpha<MESSAGE_TYPE_LENGTH>,
        nanoseconds: Numeric<NANOSECONDS_LENGTH>,
        order_id: Numeric<ORDER_ID_LENGTH>,
        order_book_id: Numeric<ORDER_BOOK_ID_LENGTH>,
        side: Alpha<SIDE_LENGTH>,
        executed_quantity: Numeric<EXECUTED_QUANTITY_LENGTH>,
        match_id: Numeric<MATCH_ID_LENGTH>,
        owner_participant_id: Alpha<OWNER_PARTICIPANT_ID_LENGTH>,
        counterparty_participant_id: Alpha<COUNTERPARTY_PARTICIPANT_ID_LENGTH>,
        trade_price: Price,
        occurred_at_cross: Alpha<OCCURRED_AT_CROSS_LENGTH>,
        printable: Alpha<PRINTABLE_LENGTH>
    ) -> Result<Self, DataFeedError> {
        let message = OrderExecutedWithPriceMessage {
            message_type,
            nanoseconds,
            order_id,
            order_book_id,
            side,
            executed_quantity,
            match_id,
            owner_participant_id,
            counterparty_participant_id,
            trade_price,
            occurred_at_cross,
            printable
        };

        message.validate_fields()?;

        Ok(message)
    }
}

impl TMessage for OrderExecutedWithPriceMessage {
    fn to_bin<const SIZE: usize>(&self) -> Result<[u8; SIZE], DataFeedError> {
        if SIZE != global_constants::ORDER_EXECUTED_WITH_PRICE_MESSAGE_BYTE_COUNT {
            return Err(DataFeedError::InvalidMessageSize(global_constants::ORDER_EXECUTED_WITH_PRICE_MESSAGE_BYTE_COUNT, SIZE));
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
        buffer_utils::write_num(&mut binary, TRADE_PRICE_BYTE_OFFSET, &self.trade_price.value.to_be_bytes());
        buffer_utils::write_alpha(&mut binary, OCCURRED_AT_CROSS_BYTE_OFFSET, &self.occurred_at_cross.value);
        buffer_utils::write_alpha(&mut binary, PRINTABLE_BYTE_OFFSET, &self.printable.value);

        Ok(binary)
    }

    fn from_bin<const SIZE: usize>(binary_data: &[u8; SIZE]) -> Result<Self, DataFeedError> where Self: Sized {
        if SIZE != global_constants::ORDER_EXECUTED_WITH_PRICE_MESSAGE_BYTE_COUNT {
            return Err(DataFeedError::InvalidMessageSize(global_constants::ORDER_EXECUTED_WITH_PRICE_MESSAGE_BYTE_COUNT, SIZE));
        }

        Ok(
            OrderExecutedWithPriceMessage::new(
                buffer_utils::read_alpha(binary_data, MESSAGE_TYPE_BYTE_OFFSET)?, 
                buffer_utils::read_numeric(binary_data, NANOSECONDS_BYTE_OFFSET)?, 
                buffer_utils::read_numeric(binary_data, ORDER_ID_BYTE_OFFSET)?, 
                buffer_utils::read_numeric(binary_data, ORDER_BOOK_ID_BYTE_OFFSET)?, 
                buffer_utils::read_alpha(binary_data, SIDE_BYTE_OFFSET)?, 
                buffer_utils::read_numeric(binary_data, EXECUTED_QUANTITY_BYTE_OFFSET)?, 
                buffer_utils::read_numeric(binary_data, MATCH_ID_BYTE_OFFSET)?, 
                buffer_utils::read_alpha(binary_data, OWNER_PARTICIPANT_ID_BYTE_OFFSET)?, 
                buffer_utils::read_alpha(binary_data, COUNTERPARTY_PARTICIPANT_ID_BYTE_OFFSET)?, 
                buffer_utils::read_price(binary_data, TRADE_PRICE_BYTE_OFFSET), 
                buffer_utils::read_alpha(binary_data, OCCURRED_AT_CROSS_BYTE_OFFSET)?, 
                buffer_utils::read_alpha(binary_data, PRINTABLE_BYTE_OFFSET)?
            )?
        )
    }

    fn validate_fields(&self) -> Result<(), DataFeedError> {
        if self.message_type.value != ['C'] {
            return Err(DataFeedError::InvalidMessageType('C', self.message_type.value[0]));
        }

        if !VALID_SIDE_VALUES.contains(&self.side.value[0]) {
            return Err(DataFeedError::InvalidSideValue(self.side.value[0]));
        }

        if !VALID_OCCURRED_AT_CROSS_VALUES.contains(&self.occurred_at_cross.value[0]) {
            return Err(DataFeedError::InvalidOccurredAtCrossValue(self.occurred_at_cross.value[0]));
        }

        if !VALID_PRINTABLE_VALUES.contains(&self.printable.value[0]) {
            return Err(DataFeedError::InvalidPrintableValue(self.printable.value[0]));
        }

        Ok(())
    }
}