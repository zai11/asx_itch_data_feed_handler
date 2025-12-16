use crate::{enums::data_feed_error::DataFeedError, global_constants, models::data_types::{alpha::Alpha, numeric::Numeric, price::Price}, traits::message::TMessage, util::buffer_utils};

const VALID_SIDE_VALUES: [char; 2] = ['B', 'S'];
const VALID_LOT_TYPE_VALUES: [u8; 5] = [0, 1, 2, 3, 4];
const VALID_EXCHANGE_ORDER_TYPE_VALUES: [u16; 15] = [4, 8, 32, 8192, (4 | 8), (4 | 32), (4 | 8192), (8 | 32), (8 | 8192), (32 | 8192), (4 | 8 | 32), (4 | 8 | 8192), (4 | 32 | 8192), (8 | 32 | 8192), (4 | 8 | 32 | 8192)];

const MESSAGE_TYPE_BYTE_OFFSET: usize = 0;
const NANOSECONDS_BYTE_OFFSET: usize = 1;
const ORDER_ID_BYTE_OFFSET: usize = 5;
const ORDER_BOOK_ID_BYTE_OFFSET: usize = 13;
const SIDE_BYTE_OFFSET: usize = 17;
const ORDER_BOOK_POSITION_BYTE_OFFSET: usize = 18;
const QUANTITY_BYTE_OFFSET: usize = 22;
const PRICE_BYTE_OFFSET: usize = 30;
const EXCHANGE_ORDER_TYPE_BYTE_OFFSET: usize = 34;
const LOT_TYPE_BYTE_OFFSET: usize = 36;
const PARTICIPANT_ID_BYTE_OFFSET: usize = 37;

const MESSAGE_TYPE_LENGTH: usize = 1;
const NANOSECONDS_LENGTH: usize = 4;
const ORDER_ID_LENGTH: usize = 8;
const ORDER_BOOK_ID_LENGTH: usize = 4;
const SIDE_LENGTH: usize = 1;
const ORDER_BOOK_POSITION_LENGTH: usize = 4;
const QUANTITY_LENGTH: usize = 8;
const EXCHANGE_ORDER_TYPE_LENGTH: usize = 2;
const LOT_TYPE_LENGTH: usize = 1;
const PARTICIPANT_ID_LENGTH: usize = 7;

pub struct AddOrderWithParticipantIdMessage {
    pub message_type: Alpha<MESSAGE_TYPE_LENGTH>,
    pub nanoseconds: Numeric<NANOSECONDS_LENGTH>,
    pub order_id: Numeric<ORDER_ID_LENGTH>,
    pub order_book_id: Numeric<ORDER_BOOK_ID_LENGTH>,
    pub side: Alpha<SIDE_LENGTH>,
    pub order_book_position: Numeric<ORDER_BOOK_POSITION_LENGTH>,
    pub quantity: Numeric<QUANTITY_LENGTH>,
    pub price: Price,
    pub exchange_order_type: Numeric<EXCHANGE_ORDER_TYPE_LENGTH>,
    pub lot_type: Numeric<LOT_TYPE_LENGTH>,
    pub participant_id: Alpha<PARTICIPANT_ID_LENGTH>
}

impl AddOrderWithParticipantIdMessage {
    pub fn new(
        message_type: Alpha<MESSAGE_TYPE_LENGTH>,
        nanoseconds: Numeric<NANOSECONDS_LENGTH>,
        order_id: Numeric<ORDER_ID_LENGTH>,
        order_book_id: Numeric<ORDER_BOOK_ID_LENGTH>,
        side: Alpha<SIDE_LENGTH>,
        order_book_position: Numeric<ORDER_BOOK_POSITION_LENGTH>,
        quantity: Numeric<QUANTITY_LENGTH>,
        price: Price,
        exchange_order_type: Numeric<EXCHANGE_ORDER_TYPE_LENGTH>,
        lot_type: Numeric<LOT_TYPE_LENGTH>,
        participant_id: Alpha<PARTICIPANT_ID_LENGTH>
    ) -> Result<Self, DataFeedError> {
        let message = AddOrderWithParticipantIdMessage {
            message_type,
            nanoseconds,
            order_id,
            order_book_id,
            side,
            order_book_position,
            quantity,
            price,
            exchange_order_type,
            lot_type,
            participant_id
        };

        message.validate_fields()?;

        Ok(message)
    }
}

impl TMessage for AddOrderWithParticipantIdMessage {
    fn to_bin<const SIZE: usize>(&self) -> Result<[u8; SIZE], DataFeedError> {
        if SIZE != global_constants::ADD_ORDER_WITH_PARTICIPANT_ID_MESSAGE_BYTE_COUNT {
            return Err(DataFeedError::InvalidMessageSize(global_constants::ADD_ORDER_WITH_PARTICIPANT_ID_MESSAGE_BYTE_COUNT, SIZE));
        }

        let mut binary = [0u8; SIZE];

        buffer_utils::write_alpha(&mut binary, MESSAGE_TYPE_BYTE_OFFSET, &self.message_type.value);
        buffer_utils::write_num(&mut binary, NANOSECONDS_BYTE_OFFSET, &self.nanoseconds.value.to_be_bytes());
        buffer_utils::write_num(&mut binary, ORDER_ID_BYTE_OFFSET, &self.order_id.value.to_be_bytes());
        buffer_utils::write_num(&mut binary, ORDER_BOOK_ID_BYTE_OFFSET, &self.order_book_id.value.to_be_bytes());
        buffer_utils::write_alpha(&mut binary, SIDE_BYTE_OFFSET, &self.side.value);
        buffer_utils::write_num(&mut binary, ORDER_BOOK_POSITION_BYTE_OFFSET, &self.order_book_position.value.to_be_bytes());
        buffer_utils::write_num(&mut binary, QUANTITY_BYTE_OFFSET, &self.quantity.value.to_be_bytes());
        buffer_utils::write_num(&mut binary, PRICE_BYTE_OFFSET, &self.price.value.to_be_bytes());
        buffer_utils::write_num(&mut binary, EXCHANGE_ORDER_TYPE_BYTE_OFFSET, &self.exchange_order_type.value.to_be_bytes());
        buffer_utils::write_num(&mut binary, LOT_TYPE_BYTE_OFFSET, &self.lot_type.value.to_be_bytes());
        buffer_utils::write_alpha(&mut binary, PARTICIPANT_ID_BYTE_OFFSET, &self.participant_id.value);

        Ok(binary)
    }

    fn from_bin<const SIZE: usize>(binary_data: &[u8; SIZE]) -> Result<Self, DataFeedError> where Self: Sized {
        if SIZE != global_constants::ADD_ORDER_WITH_PARTICIPANT_ID_MESSAGE_BYTE_COUNT {
            return Err(DataFeedError::InvalidMessageSize(global_constants::ADD_ORDER_WITH_PARTICIPANT_ID_MESSAGE_BYTE_COUNT, SIZE));
        }

        Ok(
            AddOrderWithParticipantIdMessage::new(
                buffer_utils::read_alpha(binary_data, MESSAGE_TYPE_BYTE_OFFSET)?,
                buffer_utils::read_numeric(binary_data, NANOSECONDS_BYTE_OFFSET)?,
                buffer_utils::read_numeric(binary_data, ORDER_ID_BYTE_OFFSET)?,
                buffer_utils::read_numeric(binary_data, ORDER_BOOK_ID_BYTE_OFFSET)?,
                buffer_utils::read_alpha(binary_data, SIDE_BYTE_OFFSET)?,
                buffer_utils::read_numeric(binary_data, ORDER_BOOK_POSITION_BYTE_OFFSET)?,
                buffer_utils::read_numeric(binary_data, QUANTITY_BYTE_OFFSET)?,
                buffer_utils::read_price(binary_data, PRICE_BYTE_OFFSET),
                buffer_utils::read_numeric(binary_data, EXCHANGE_ORDER_TYPE_BYTE_OFFSET)?,
                buffer_utils::read_numeric(binary_data, LOT_TYPE_BYTE_OFFSET)?,
                buffer_utils::read_alpha(binary_data, PARTICIPANT_ID_BYTE_OFFSET)?
            )?
        )
    }

    fn validate_fields(&self) -> Result<(), DataFeedError> {
        if self.message_type.value == ['F'] {
            return Err(DataFeedError::InvalidMessageType('F', self.message_type.value[0]));
        }

        if !VALID_SIDE_VALUES.contains(&self.side.value[0]) {
            return Err(DataFeedError::InvalidSideValue(self.side.value[0]));
        }

        if !VALID_EXCHANGE_ORDER_TYPE_VALUES.contains(&(self.exchange_order_type.value as u16)) {
            return Err(DataFeedError::InvalidExchangeOrderTypeValue(self.exchange_order_type.value));
        }

        if !VALID_LOT_TYPE_VALUES.contains(&(self.lot_type.value as u8)) {
            return Err(DataFeedError::InvalidLotTypeValue(self.lot_type.value));
        }

        Ok(())
    }
}