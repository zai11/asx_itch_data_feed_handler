use crate::{enums::data_feed_error::DataFeedError, global_constants, models::data_types::{alpha::Alpha, numeric::Numeric, price::Price}, traits::message::TMessage, util::buffer_utils};

const VALID_SIDE_VALUES: [char; 3] = ['B', 'S', ' '];
const VALID_PRINTABLE_VALUES: [char; 2] = ['N', 'Y'];
const VALID_OCCURRED_AT_CROSS_VALUES: [char; 2] = ['N', 'Y'];

const MESSAGE_TYPE_BYTE_OFFSET: usize = 0;
const NANOSECONDS_BYTE_OFFSET: usize = 1;
const MATCH_ID_BYTE_OFFSET: usize = 5;
const SIDE_BYTE_OFFSET: usize = 17;
const QUANTITY_BYTE_OFFSET: usize = 18;
const ORDER_BOOK_ID_BYTE_OFFSET: usize = 26;
const TRADE_PRICE_BYTE_OFFSET: usize = 30;
const OWNER_PARTICIPANT_ID_BYTE_OFFSET: usize = 34;
const COUNTERPARTY_PARTICIPANT_ID_BYTE_OFFSET: usize = 41;
const PRINTABLE_BYTE_OFFSET: usize = 48;
const OCCURRED_AT_CROSS_BYTE_OFFSET: usize = 49;

const MESSAGE_TYPE_LENGTH: usize = 1;
const NANOSECONDS_LENGTH: usize = 4;
const MATCH_ID_LENGTH: usize = 12;
const SIDE_LENGTH: usize = 1;
const QUANTITY_LENGTH: usize = 8;
const ORDER_BOOK_ID_LENTH: usize = 4;
const OWNER_PARTICIPANT_ID_LENGTH: usize = 7;
const COUNTERPARTY_PARTICIPANT_ID_LENGTH: usize = 7;
const PRINTABLE_LENGTH: usize = 1;
const OCCURRED_AT_CROSS_LENGTH: usize = 1;

pub struct TradeMessage {
    pub message_type: Alpha<MESSAGE_TYPE_LENGTH>,
    pub nanoseconds: Numeric<NANOSECONDS_LENGTH>,
    pub match_id: Numeric<MATCH_ID_LENGTH>,
    pub side: Alpha<SIDE_LENGTH>,
    pub quantity: Numeric<QUANTITY_LENGTH>,
    pub order_book_id: Numeric<ORDER_BOOK_ID_LENTH>,
    pub trade_price: Price,
    pub owner_participant_id: Alpha<OWNER_PARTICIPANT_ID_LENGTH>,
    pub counterparty_participant_id: Alpha<COUNTERPARTY_PARTICIPANT_ID_LENGTH>,
    pub printable: Alpha<PRINTABLE_LENGTH>,
    pub occurred_at_cross: Alpha<OCCURRED_AT_CROSS_LENGTH>
}

impl TradeMessage {
    pub fn new(
        message_type: Alpha<MESSAGE_TYPE_LENGTH>,
        nanoseconds: Numeric<NANOSECONDS_LENGTH>,
        match_id: Numeric<MATCH_ID_LENGTH>,
        side: Alpha<SIDE_LENGTH>,
        quantity: Numeric<QUANTITY_LENGTH>,
        order_book_id: Numeric<ORDER_BOOK_ID_LENTH>,
        trade_price: Price,
        owner_participant_id: Alpha<OWNER_PARTICIPANT_ID_LENGTH>,
        counterparty_participant_id: Alpha<COUNTERPARTY_PARTICIPANT_ID_LENGTH>,
        printable: Alpha<PRINTABLE_LENGTH>,
        occurred_at_cross: Alpha<OCCURRED_AT_CROSS_LENGTH>
    ) -> Result<Self, DataFeedError> {
        let message = TradeMessage {
            message_type,
            nanoseconds,
            match_id,
            side,
            quantity,
            order_book_id,
            trade_price,
            owner_participant_id,
            counterparty_participant_id,
            printable,
            occurred_at_cross
        };

        message.validate_fields()?;

        Ok(message)
    }
}

impl TMessage for TradeMessage {
    fn to_bin<const SIZE: usize>(&self) -> Result<[u8; SIZE], DataFeedError> {
        if SIZE != global_constants::TRADE_MESSAGE_BYTE_COUNT {
            return Err(DataFeedError::InvalidMessageSize(global_constants::TRADE_MESSAGE_BYTE_COUNT, SIZE));
        }

        let mut binary = [0u8; SIZE];

        buffer_utils::write_alpha(&mut binary, MESSAGE_TYPE_BYTE_OFFSET, &self.message_type.value);
        buffer_utils::write_num(&mut binary, NANOSECONDS_BYTE_OFFSET, &self.nanoseconds.value.to_be_bytes());
        buffer_utils::write_num(&mut binary, MATCH_ID_BYTE_OFFSET, &self.match_id.value.to_be_bytes());
        buffer_utils::write_alpha(&mut binary, SIDE_BYTE_OFFSET, &self.side.value);
        buffer_utils::write_num(&mut binary, QUANTITY_BYTE_OFFSET, &self.quantity.value.to_be_bytes());
        buffer_utils::write_num(&mut binary, ORDER_BOOK_ID_BYTE_OFFSET, &self.order_book_id.value.to_be_bytes());
        buffer_utils::write_num(&mut binary, TRADE_PRICE_BYTE_OFFSET, &self.trade_price.value.to_be_bytes());
        buffer_utils::write_alpha(&mut binary, OWNER_PARTICIPANT_ID_BYTE_OFFSET, &self.owner_participant_id.value);
        buffer_utils::write_alpha(&mut binary, COUNTERPARTY_PARTICIPANT_ID_BYTE_OFFSET, &self.counterparty_participant_id.value);
        buffer_utils::write_alpha(&mut binary, PRINTABLE_BYTE_OFFSET, &self.printable.value);
        buffer_utils::write_alpha(&mut binary, OCCURRED_AT_CROSS_BYTE_OFFSET, &self.occurred_at_cross.value);

        Ok(binary)
    }

    fn from_bin<const SIZE: usize>(binary_data: &[u8; SIZE]) -> Result<Self, DataFeedError> where Self: Sized {
        if SIZE != global_constants::TRADE_MESSAGE_BYTE_COUNT {
            return Err(DataFeedError::InvalidMessageSize(global_constants::TRADE_MESSAGE_BYTE_COUNT, SIZE));
        }

        Ok(
            TradeMessage::new(
                buffer_utils::read_alpha(binary_data, MESSAGE_TYPE_BYTE_OFFSET)?, 
                buffer_utils::read_numeric(binary_data, NANOSECONDS_BYTE_OFFSET)?, 
                buffer_utils::read_numeric(binary_data, MATCH_ID_BYTE_OFFSET)?, 
                buffer_utils::read_alpha(binary_data, SIDE_BYTE_OFFSET)?, 
                buffer_utils::read_numeric(binary_data, QUANTITY_BYTE_OFFSET)?, 
                buffer_utils::read_numeric(binary_data, ORDER_BOOK_ID_BYTE_OFFSET)?, 
                buffer_utils::read_price(binary_data, TRADE_PRICE_BYTE_OFFSET), 
                buffer_utils::read_alpha(binary_data, OWNER_PARTICIPANT_ID_BYTE_OFFSET)?, 
                buffer_utils::read_alpha(binary_data, COUNTERPARTY_PARTICIPANT_ID_BYTE_OFFSET)?, 
                buffer_utils::read_alpha(binary_data, PRINTABLE_BYTE_OFFSET)?, 
                buffer_utils::read_alpha(binary_data, OCCURRED_AT_CROSS_BYTE_OFFSET)?
            )?
        )
    }

    fn validate_fields(&self) -> Result<(), DataFeedError> {
        if self.message_type.value != ['P'] {
            return Err(DataFeedError::InvalidMessageType('P', self.message_type.value[0]));
        }

        if !VALID_SIDE_VALUES.contains(&self.side.value[0]) {
            return Err(DataFeedError::InvalidSideValue(self.side.value[0]));
        }

        if !VALID_PRINTABLE_VALUES.contains(&self.printable.value[0]) {
            return Err(DataFeedError::InvalidPrintableValue(self.printable.value[0]));
        }

        if !VALID_OCCURRED_AT_CROSS_VALUES.contains(&self.occurred_at_cross.value[0]) {
            return Err(DataFeedError::InvalidOccurredAtCrossValue(self.occurred_at_cross.value[0]));
        }

        Ok(())
    }
}