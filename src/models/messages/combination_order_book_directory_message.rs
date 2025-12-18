use crate::{enums::data_feed_error::DataFeedError, global_constants, models::data_types::{alpha::Alpha, numeric::Numeric}, traits::message::TMessage, util::buffer_utils};

const VALID_FINANCIAL_PRODUCT_VALUES: [u8; 4] = [1, 3, 5, 11];
const VALID_LEG_1_AND_2_SIDE_VALUES: [char; 2] = ['B', 'C'];
const VALID_LEG_3_AND_4_SIDE_VALUES: [char; 3] = ['B', 'C', '?'];

const MESSAGE_TYPE_BYTE_OFFSET: usize = 0;
const NANOSECONDS_BYTE_OFFSET: usize = 1;
const ORDER_BOOK_ID_BYTE_OFFSET: usize = 5;
const SYMBOL_BYTE_OFFSET: usize = 9;
const LONG_NAME_BYTE_OFFSET: usize = 41;
const ISIN_BYTE_OFFSET: usize = 80;
const FINANCIAL_PRODUCT_BYTE_OFFSET: usize = 92;
const TRADING_CURRENCY_BYTE_OFFSET: usize = 93;
const NUMBER_OF_DECIMALS_IN_PRICE_BYTE_OFFSET: usize = 96;
const NUMBER_OF_DECIMALS_IN_NOMINAL_VALUE_BYTE_OFFSET: usize = 98;
const ODD_LOT_SIZE_BYTE_OFFSET: usize = 100;
const ROUND_LOT_SIZE_BYTE_OFFSET: usize = 104;
const BLOCK_LOT_SIZE_BYTE_OFFSET: usize = 108;
const NOMINAL_VALUE_BYTE_OFFSET: usize = 112;
const LEG_1_SYMBOL_BYTE_OFFSET: usize = 120;
const LEG_1_SIDE_BYTE_OFFSET: usize = 152;
const LEG_1_RATIO_BYTE_OFFSET: usize = 153;
const LEG_2_SYMBOL_BYTE_OFFSET: usize = 157;
const LEG_2_SIDE_BYTE_OFFSET: usize = 189;
const LEG_2_RATIO_BYTE_OFFSET: usize = 190;
const LEG_3_SYMBOL_BYTE_OFFSET: usize = 194;
const LEG_3_SIDE_BYTE_OFFSET: usize = 226;
const LEG_3_RATIO_BYTE_OFFSET: usize = 227;
const LEG_4_SYMBOL_BYTE_OFFSET: usize = 231;
const LEG_4_SIDE_BYTE_OFFSET: usize = 263;
const LEG_4_RATIO_BYTE_OFFSET: usize = 264;

const MESSAGE_TYPE_LENGTH: usize = 1;
const NANOSECONDS_LENGTH: usize = 4;
const ORDER_BOOK_ID_LENGTH: usize = 4;
const SYMBOL_LENGTH: usize = 32;
const LONG_NAME_LENGTH: usize = 32;
const ISIN_LENGTH: usize = 12;
const FINANCIAL_PRODUCT_LENGTH: usize = 1;
const TRADING_CURRENCY_LENGTH: usize = 3;
const NUMBER_OF_DECIMALS_IN_PRICE_LENGTH: usize = 2;
const NUMBER_OF_DECIMALS_IN_NOMINAL_VALUE_LENGTH: usize = 2;
const ODD_LOT_SIZE_LENGTH: usize = 4;
const ROUND_LOT_SIZE_LENGTH: usize = 4;
const BLOCK_LOT_SIZE_LENGTH: usize = 4;
const NOMINAL_VALUE_LENGTH: usize = 8;
const LEG_1_SYMBOL_LENGTH: usize = 32;
const LEG_1_SIDE_LENGTH: usize = 1;
const LEG_1_RATIO_LENGH: usize = 4;
const LEG_2_SYMBOL_LENGTH: usize = 32;
const LEG_2_SIDE_LENGTH: usize = 1;
const LEG_2_RATIO_LENGTH: usize = 4;
const LEG_3_SYMBOL_LENGTH: usize = 32;
const LEG_3_SIDE_LENGTH: usize = 1;
const LEG_3_RATIO_LENGTH: usize = 4;
const LEG_4_SYMBOL_LENGTH: usize = 32;
const LEG_4_SIDE_LENGTH: usize = 1;
const LEG_4_RATIO_LENGTH: usize = 4;

pub struct CombinationOrderBookDirectoryMessage {
    pub message_type: Alpha<MESSAGE_TYPE_LENGTH>,
    pub nanoseconds: Numeric<NANOSECONDS_LENGTH>,
    pub order_book_id: Numeric<ORDER_BOOK_ID_LENGTH>,
    pub symbol: Alpha<SYMBOL_LENGTH>,
    pub long_name: Alpha<LONG_NAME_LENGTH>,
    pub isin: Alpha<ISIN_LENGTH>,
    pub financial_product: Numeric<FINANCIAL_PRODUCT_LENGTH>,
    pub trading_currency: Alpha<TRADING_CURRENCY_LENGTH>,
    pub number_of_decimals_in_price: Numeric<NUMBER_OF_DECIMALS_IN_PRICE_LENGTH>,
    pub number_of_decimals_in_nominal_value: Numeric<NUMBER_OF_DECIMALS_IN_NOMINAL_VALUE_LENGTH>,
    pub odd_lot_size: Numeric<ODD_LOT_SIZE_LENGTH>,
    pub round_lot_size: Numeric<ROUND_LOT_SIZE_LENGTH>,
    pub block_lot_size: Numeric<BLOCK_LOT_SIZE_LENGTH>,
    pub nominal_value: Numeric<NOMINAL_VALUE_LENGTH>,
    pub leg_1_symbol: Alpha<LEG_1_SYMBOL_LENGTH>,
    pub leg_1_side: Alpha<LEG_1_SIDE_LENGTH>,
    pub leg_1_ratio: Numeric<LEG_1_RATIO_LENGH>,
    pub leg_2_symbol: Alpha<LEG_2_SYMBOL_LENGTH>,
    pub leg_2_side: Alpha<LEG_2_SIDE_LENGTH>,
    pub leg_2_ratio: Numeric<LEG_2_RATIO_LENGTH>,
    pub leg_3_symbol: Alpha<LEG_3_SYMBOL_LENGTH>,
    pub leg_3_side: Alpha<LEG_3_SIDE_LENGTH>,
    pub leg_3_ratio: Numeric<LEG_3_RATIO_LENGTH>,
    pub leg_4_symbol: Alpha<LEG_4_SYMBOL_LENGTH>,
    pub leg_4_side: Alpha<LEG_4_SIDE_LENGTH>,
    pub leg_4_ratio: Numeric<LEG_4_RATIO_LENGTH>
}

impl CombinationOrderBookDirectoryMessage {
    pub fn new(
        message_type: Alpha<MESSAGE_TYPE_LENGTH>,
        nanoseconds: Numeric<NANOSECONDS_LENGTH>,
        order_book_id: Numeric<ORDER_BOOK_ID_LENGTH>,
        symbol: Alpha<SYMBOL_LENGTH>,
        long_name: Alpha<LONG_NAME_LENGTH>,
        isin: Alpha<ISIN_LENGTH>,
        financial_product: Numeric<FINANCIAL_PRODUCT_LENGTH>,
        trading_currency: Alpha<TRADING_CURRENCY_LENGTH>,
        number_of_decimals_in_price: Numeric<NUMBER_OF_DECIMALS_IN_PRICE_LENGTH>,
        number_of_decimals_in_nominal_value: Numeric<NUMBER_OF_DECIMALS_IN_NOMINAL_VALUE_LENGTH>,
        odd_lot_size: Numeric<ODD_LOT_SIZE_LENGTH>,
        round_lot_size: Numeric<ROUND_LOT_SIZE_LENGTH>,
        block_lot_size: Numeric<BLOCK_LOT_SIZE_LENGTH>,
        nominal_value: Numeric<NOMINAL_VALUE_LENGTH>,
        leg_1_symbol: Alpha<LEG_1_SYMBOL_LENGTH>,
        leg_1_side: Alpha<LEG_1_SIDE_LENGTH>,
        leg_1_ratio: Numeric<LEG_1_RATIO_LENGH>,
        leg_2_symbol: Alpha<LEG_2_SYMBOL_LENGTH>,
        leg_2_side: Alpha<LEG_2_SIDE_LENGTH>,
        leg_2_ratio: Numeric<LEG_2_RATIO_LENGTH>,
        leg_3_symbol: Alpha<LEG_3_SYMBOL_LENGTH>,
        leg_3_side: Alpha<LEG_3_SIDE_LENGTH>,
        leg_3_ratio: Numeric<LEG_3_RATIO_LENGTH>,
        leg_4_symbol: Alpha<LEG_4_SYMBOL_LENGTH>,
        leg_4_side: Alpha<LEG_4_SIDE_LENGTH>,
        leg_4_ratio: Numeric<LEG_4_RATIO_LENGTH>
    ) -> Result<Self, DataFeedError> {
        let message = CombinationOrderBookDirectoryMessage {
            message_type,
            nanoseconds,
            order_book_id,
            symbol,
            long_name,
            isin,
            financial_product,
            trading_currency,
            number_of_decimals_in_price,
            number_of_decimals_in_nominal_value,
            odd_lot_size,
            round_lot_size,
            block_lot_size,
            nominal_value,
            leg_1_symbol,
            leg_1_side,
            leg_1_ratio,
            leg_2_symbol,
            leg_2_side,
            leg_2_ratio,
            leg_3_symbol,
            leg_3_side,
            leg_3_ratio,
            leg_4_symbol,
            leg_4_side,
            leg_4_ratio
        };

        message.validate_fields()?;

        Ok(message)
    }
}

impl TMessage for CombinationOrderBookDirectoryMessage {
    fn to_bin<const SIZE: usize>(&self) -> Result<[u8; SIZE], DataFeedError> {
        if SIZE != global_constants::COMBINATION_ORDER_BOOK_DIRECTORY_MESSAGE_BYTE_COUNT {
            return Err(DataFeedError::InvalidMessageSize(global_constants::COMBINATION_ORDER_BOOK_DIRECTORY_MESSAGE_BYTE_COUNT, SIZE));
        }

        let mut binary = [0u8; SIZE];

        buffer_utils::write_alpha(&mut binary, MESSAGE_TYPE_BYTE_OFFSET, &self.message_type.value);
        buffer_utils::write_num(&mut binary, NANOSECONDS_BYTE_OFFSET, &self.nanoseconds.value.to_be_bytes());
        buffer_utils::write_num(&mut binary, ORDER_BOOK_ID_BYTE_OFFSET, &self.order_book_id.value.to_be_bytes());
        buffer_utils::write_alpha(&mut binary, SYMBOL_BYTE_OFFSET, &self.symbol.value);
        buffer_utils::write_alpha(&mut binary, LONG_NAME_BYTE_OFFSET, &self.long_name.value);
        buffer_utils::write_alpha(&mut binary, ISIN_BYTE_OFFSET, &self.isin.value);
        buffer_utils::write_num(&mut binary, FINANCIAL_PRODUCT_BYTE_OFFSET, &self.financial_product.value.to_be_bytes());
        buffer_utils::write_alpha(&mut binary, TRADING_CURRENCY_BYTE_OFFSET, &self.trading_currency.value);
        buffer_utils::write_num(&mut binary, NUMBER_OF_DECIMALS_IN_PRICE_BYTE_OFFSET, &self.number_of_decimals_in_price.value.to_be_bytes());
        buffer_utils::write_num(&mut binary, NUMBER_OF_DECIMALS_IN_NOMINAL_VALUE_BYTE_OFFSET, &self.number_of_decimals_in_nominal_value.value.to_be_bytes());
        buffer_utils::write_num(&mut binary, ODD_LOT_SIZE_BYTE_OFFSET, &self.odd_lot_size.value.to_be_bytes());
        buffer_utils::write_num(&mut binary, ROUND_LOT_SIZE_BYTE_OFFSET, &self.round_lot_size.value.to_be_bytes());
        buffer_utils::write_num(&mut binary, BLOCK_LOT_SIZE_BYTE_OFFSET, &self.block_lot_size.value.to_be_bytes());
        buffer_utils::write_num(&mut binary, NOMINAL_VALUE_BYTE_OFFSET, &self.nominal_value.value.to_be_bytes());
        buffer_utils::write_alpha(&mut binary, LEG_1_SYMBOL_BYTE_OFFSET, &self.leg_1_symbol.value);
        buffer_utils::write_alpha(&mut binary, LEG_1_SIDE_BYTE_OFFSET, &self.leg_1_side.value);
        buffer_utils::write_num(&mut binary, LEG_1_RATIO_BYTE_OFFSET, &self.leg_1_ratio.value.to_be_bytes());
        buffer_utils::write_alpha(&mut binary, LEG_2_SYMBOL_BYTE_OFFSET, &self.leg_2_symbol.value);
        buffer_utils::write_alpha(&mut binary, LEG_2_SIDE_BYTE_OFFSET, &self.leg_2_side.value);
        buffer_utils::write_num(&mut binary, LEG_2_RATIO_BYTE_OFFSET, &self.leg_2_ratio.value.to_be_bytes());
        buffer_utils::write_alpha(&mut binary, LEG_3_SYMBOL_BYTE_OFFSET, &self.leg_3_symbol.value);
        buffer_utils::write_alpha(&mut binary, LEG_3_SIDE_BYTE_OFFSET, &self.leg_3_side.value);
        buffer_utils::write_num(&mut binary, LEG_3_RATIO_BYTE_OFFSET, &self.leg_3_ratio.value.to_be_bytes());
        buffer_utils::write_alpha(&mut binary, LEG_4_SYMBOL_BYTE_OFFSET, &self.leg_4_symbol.value);
        buffer_utils::write_alpha(&mut binary, LEG_4_SIDE_BYTE_OFFSET, &self.leg_4_side.value);
        buffer_utils::write_num(&mut binary, LEG_4_RATIO_BYTE_OFFSET, &self.leg_4_ratio.value.to_be_bytes());

        Ok(binary)
    }

    fn from_bin<const SIZE: usize>(binary_data: &[u8; SIZE]) -> Result<Self, DataFeedError> where Self: Sized {
        if SIZE != global_constants::COMBINATION_ORDER_BOOK_DIRECTORY_MESSAGE_BYTE_COUNT {
            return Err(DataFeedError::InvalidMessageSize(global_constants::COMBINATION_ORDER_BOOK_DIRECTORY_MESSAGE_BYTE_COUNT, SIZE));
        }

        Ok(
            CombinationOrderBookDirectoryMessage::new(
                buffer_utils::read_alpha(binary_data, MESSAGE_TYPE_BYTE_OFFSET)?,
                buffer_utils::read_numeric(binary_data, NANOSECONDS_BYTE_OFFSET)?,
                buffer_utils::read_numeric(binary_data, ORDER_BOOK_ID_BYTE_OFFSET)?,
                buffer_utils::read_alpha(binary_data, SYMBOL_BYTE_OFFSET)?,
                buffer_utils::read_alpha(binary_data, LONG_NAME_BYTE_OFFSET)?,
                buffer_utils::read_alpha(binary_data, ISIN_BYTE_OFFSET)?,
                buffer_utils::read_numeric(binary_data, FINANCIAL_PRODUCT_BYTE_OFFSET)?,
                buffer_utils::read_alpha(binary_data, TRADING_CURRENCY_BYTE_OFFSET)?,
                buffer_utils::read_numeric(binary_data, NUMBER_OF_DECIMALS_IN_PRICE_BYTE_OFFSET)?,
                buffer_utils::read_numeric(binary_data, NUMBER_OF_DECIMALS_IN_NOMINAL_VALUE_BYTE_OFFSET)?,
                buffer_utils::read_numeric(binary_data, ODD_LOT_SIZE_BYTE_OFFSET)?,
                buffer_utils::read_numeric(binary_data, ROUND_LOT_SIZE_BYTE_OFFSET)?,
                buffer_utils::read_numeric(binary_data, BLOCK_LOT_SIZE_BYTE_OFFSET)?,
                buffer_utils::read_numeric(binary_data, NOMINAL_VALUE_BYTE_OFFSET)?,
                buffer_utils::read_alpha(binary_data, LEG_1_SYMBOL_BYTE_OFFSET)?,
                buffer_utils::read_alpha(binary_data, LEG_1_SIDE_BYTE_OFFSET)?,
                buffer_utils::read_numeric(binary_data, LEG_1_RATIO_BYTE_OFFSET)?,
                buffer_utils::read_alpha(binary_data, LEG_2_SYMBOL_BYTE_OFFSET)?,
                buffer_utils::read_alpha(binary_data, LEG_2_SIDE_BYTE_OFFSET)?,
                buffer_utils::read_numeric(binary_data, LEG_2_RATIO_BYTE_OFFSET)?,
                buffer_utils::read_alpha(binary_data, LEG_3_SYMBOL_BYTE_OFFSET)?,
                buffer_utils::read_alpha(binary_data, LEG_3_SIDE_BYTE_OFFSET)?,
                buffer_utils::read_numeric(binary_data, LEG_3_RATIO_BYTE_OFFSET)?,
                buffer_utils::read_alpha(binary_data, LEG_4_SYMBOL_BYTE_OFFSET)?,
                buffer_utils::read_alpha(binary_data, LEG_4_SIDE_BYTE_OFFSET)?,
                buffer_utils::read_numeric(binary_data, LEG_4_RATIO_BYTE_OFFSET)?,
            )?
        )
    }
    
    fn validate_fields(&self) -> Result<(), DataFeedError> {
        if self.message_type.value != ['M'] {
            return Err(DataFeedError::InvalidMessageType('M', self.message_type.value[0]));
        }

        if !VALID_FINANCIAL_PRODUCT_VALUES.contains(&(self.financial_product.value as u8)) {
            return Err(DataFeedError::InvalidFinancialProductValue(self.financial_product.value));
        }

        if !VALID_LEG_1_AND_2_SIDE_VALUES.contains(&self.leg_1_side.value[0]) {
            return Err(DataFeedError::InvalidSideValue(self.leg_1_side.value[0]));
        }
        
        if !VALID_LEG_1_AND_2_SIDE_VALUES.contains(&self.leg_2_side.value[0]) {
            return Err(DataFeedError::InvalidSideValue(self.leg_2_side.value[0]));
        }

        if !VALID_LEG_3_AND_4_SIDE_VALUES.contains(&self.leg_3_side.value[0]) {
            return Err(DataFeedError::InvalidSideValue(self.leg_3_side.value[0]));
        }

        if !VALID_LEG_3_AND_4_SIDE_VALUES.contains(&self.leg_4_side.value[0]) {
            return Err(DataFeedError::InvalidSideValue(self.leg_4_side.value[0]));
        }

        if self.leg_1_ratio.value + self.leg_2_ratio.value + self.leg_3_ratio.value + self.leg_4_ratio.value != 1 {
            return Err(DataFeedError::InvalidLegRatioValues(self.leg_1_ratio.value, self.leg_2_ratio.value, self.leg_3_ratio.value, self.leg_4_ratio.value));
        }

        Ok(())
    }
}