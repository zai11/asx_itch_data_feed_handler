use crate::{enums::data_feed_error::DataFeedError, global_constants, models::data_types::{alpha::Alpha, numeric::Numeric, price::Price}, traits::message::TMessage, util::buffer_utils};

const MESSAGE_TYPE_BYTE_OFFSET: usize = 0;
const NANOSECONDS_BYTE_OFFSET: usize = 1;
const ORDER_BOOK_ID_BYTE_OFFSET: usize = 5;
const BID_QUANTITY_BYTE_OFFSET: usize = 9;
const ASK_QUANTITY_BYTE_OFFSET: usize = 17;
const EQUILIBRIUM_PRICE_BYTE_OFFSET: usize = 25;
const BEST_BID_PRICE_BYTE_OFFSET: usize = 29;
const BEST_ASK_PRICE_BYTE_OFFSET: usize = 33;
const BEST_BID_QUANTITY_BYTE_OFFSET: usize = 37;
const BEST_ASK_QUANTITY_BYTE_OFFSET: usize = 45;

const MESSAGE_TYPE_LENGTH: usize = 1;
const NANOSECONDS_LENGTH: usize = 4;
const ORDER_BOOK_ID_LENGTH: usize = 4;
const BID_QUANTITY_LENGTH: usize = 8;
const ASK_QUANTITY_LENGTH: usize = 8;
const BEST_BID_QUANTITY_LENGTH: usize = 8;
const BEST_ASK_QUANTITY_LENGTH: usize = 8;

pub struct EquilibriumPriceUpdateMessage {
    pub message_type: Alpha<MESSAGE_TYPE_LENGTH>,
    pub nanoseconds: Numeric<NANOSECONDS_LENGTH>,
    pub order_book_id: Numeric<ORDER_BOOK_ID_LENGTH>,
    pub bid_quantity: Numeric<BID_QUANTITY_LENGTH>,
    pub ask_quantity: Numeric<ASK_QUANTITY_LENGTH>,
    pub equilibrium_price: Price,
    pub best_bid_price: Price,
    pub best_ask_price: Price,
    pub best_bid_quantity: Numeric<BEST_BID_QUANTITY_LENGTH>,
    pub best_ask_quantity: Numeric<BEST_ASK_QUANTITY_LENGTH>
}

impl EquilibriumPriceUpdateMessage {
    pub fn new(
        message_type: Alpha<MESSAGE_TYPE_LENGTH>,
        nanoseconds: Numeric<NANOSECONDS_LENGTH>,
        order_book_id: Numeric<ORDER_BOOK_ID_LENGTH>,
        bid_quantity: Numeric<BID_QUANTITY_LENGTH>,
        ask_quantity: Numeric<ASK_QUANTITY_LENGTH>,
        equilibrium_price: Price,
        best_bid_price: Price,
        best_ask_price: Price,
        best_bid_quantity: Numeric<BEST_BID_QUANTITY_LENGTH>,
        best_ask_quantity: Numeric<BEST_ASK_QUANTITY_LENGTH>
    ) -> Result<Self, DataFeedError> {
        let message = EquilibriumPriceUpdateMessage {
            message_type,
            nanoseconds,
            order_book_id,
            bid_quantity,
            ask_quantity,
            equilibrium_price,
            best_bid_price,
            best_ask_price,
            best_bid_quantity,
            best_ask_quantity
        };

        message.validate_fields()?;

        Ok(message)
    }
}

impl TMessage for EquilibriumPriceUpdateMessage {
    fn to_bin<const SIZE: usize>(&self) -> Result<[u8; SIZE], DataFeedError> {
        if SIZE != global_constants::EQUILIBRIUM_PRICE_UPDATE_MESSAGE_BYTE_COUNT {
            return Err(DataFeedError::InvalidMessageSize(global_constants::EQUILIBRIUM_PRICE_UPDATE_MESSAGE_BYTE_COUNT, SIZE));
        }

        let mut binary = [0u8; SIZE];

        buffer_utils::write_alpha(&mut binary, MESSAGE_TYPE_BYTE_OFFSET, &self.message_type.value);
        buffer_utils::write_num(&mut binary, NANOSECONDS_BYTE_OFFSET, &self.nanoseconds.value.to_be_bytes());
        buffer_utils::write_num(&mut binary, ORDER_BOOK_ID_BYTE_OFFSET, &self.order_book_id.value.to_be_bytes());
        buffer_utils::write_num(&mut binary, BID_QUANTITY_BYTE_OFFSET, &self.bid_quantity.value.to_be_bytes());
        buffer_utils::write_num(&mut binary, ASK_QUANTITY_BYTE_OFFSET, &self.ask_quantity.value.to_be_bytes());
        buffer_utils::write_num(&mut binary, EQUILIBRIUM_PRICE_BYTE_OFFSET, &self.equilibrium_price.value.to_be_bytes());
        buffer_utils::write_num(&mut binary, BEST_BID_PRICE_BYTE_OFFSET, &self.best_bid_price.value.to_be_bytes());
        buffer_utils::write_num(&mut binary, BEST_ASK_PRICE_BYTE_OFFSET, &self.best_ask_price.value.to_be_bytes());
        buffer_utils::write_num(&mut binary, BEST_BID_QUANTITY_BYTE_OFFSET, &self.best_bid_quantity.value.to_be_bytes());
        buffer_utils::write_num(&mut binary, BEST_ASK_QUANTITY_BYTE_OFFSET, &self.best_ask_quantity.value.to_be_bytes());

        Ok(binary)
    }

    fn from_bin<const SIZE: usize>(binary_data: &[u8; SIZE]) -> Result<Self, DataFeedError> where Self: Sized {
        if SIZE != global_constants::EQUILIBRIUM_PRICE_UPDATE_MESSAGE_BYTE_COUNT {
            return Err(DataFeedError::InvalidMessageSize(global_constants::EQUILIBRIUM_PRICE_UPDATE_MESSAGE_BYTE_COUNT, SIZE));
        }

        Ok(
            EquilibriumPriceUpdateMessage::new(
                buffer_utils::read_alpha(binary_data, MESSAGE_TYPE_BYTE_OFFSET)?, 
                buffer_utils::read_numeric(binary_data, NANOSECONDS_BYTE_OFFSET)?, 
                buffer_utils::read_numeric(binary_data, ORDER_BOOK_ID_BYTE_OFFSET)?, 
                buffer_utils::read_numeric(binary_data, BID_QUANTITY_BYTE_OFFSET)?, 
                buffer_utils::read_numeric(binary_data, ASK_QUANTITY_BYTE_OFFSET)?, 
                buffer_utils::read_price(binary_data, EQUILIBRIUM_PRICE_BYTE_OFFSET), 
                buffer_utils::read_price(binary_data, BEST_BID_PRICE_BYTE_OFFSET), 
                buffer_utils::read_price(binary_data, BEST_ASK_PRICE_BYTE_OFFSET), 
                buffer_utils::read_numeric(binary_data, BEST_BID_QUANTITY_BYTE_OFFSET)?, 
                buffer_utils::read_numeric(binary_data, BEST_ASK_QUANTITY_BYTE_OFFSET)?
            )?
        )
    }

    fn validate_fields(&self) -> Result<(), DataFeedError> {
        if self.message_type.value != ['Z'] {
            return Err(DataFeedError::InvalidMessageType('Z', self.message_type.value[0]));
        }

        Ok(())
    }
}