use std::fmt::{Debug, Display};

pub enum DataFeedError {
    NonISO88591Value(char),
    InvalidNumericSize(usize),
    InvalidMessageSize(usize, usize),
    InvalidMessageType(char, char),
    InvalidSideValue(char),
    InvalidExchangeOrderTypeValue(u128),
    InvalidLotTypeValue(u128),
    InvalidFinancialProductValue(u128),
    InvalidLegRatioValues(u128, u128, u128, u128),
    InvalidOccurredAtCrossValue(char),
    InvalidPrintableValue(char),
    InvalidEventCodeValue(char),
    Other(String)
}

impl Display for DataFeedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NonISO88591Value(c) => write!(f, "A non-ISO 8859-1 value was detected in alpha field value: {c}."),
            Self::InvalidNumericSize(expected_size) => write!(f, "Attempted to set numeric field of size {expected_size} bytes with value too large."),
            Self::InvalidMessageSize(expected_message_size, actual_message_size) => write!(f, "An invalid message size was provided. Expected: {expected_message_size}, but was: {actual_message_size}."),
            Self::InvalidMessageType(expected_message_type, actual_message_type) => write!(f, "Message validation error: An invalid message type was detected. Expected '{expected_message_type}' but was '{actual_message_type}'."),
            Self::InvalidSideValue(side_value) => write!(f, "Message validation error: The specified side value '{side_value}' is not valid."),
            Self::InvalidExchangeOrderTypeValue(exchange_order_type_value) => write!(f, "Message validation error: The specified exchange order type value '{exchange_order_type_value}' is not valid."),
            Self::InvalidLotTypeValue(lot_type_value) => write!(f, "Message validation error: The specified lot type value '{lot_type_value}' is not valid."),
            Self::InvalidFinancialProductValue(financial_product_value) => write!(f, "Message validation error: The specified financial product value '{financial_product_value}' is not valid."),
            Self::InvalidLegRatioValues(leg_1_ratio_value, leg_2_ratio_value, leg_3_ratio_value, leg_4_ratio_value) => write!(f, "Message validation error: The provided leg ratios do not add to 1. Leg 1 ratio: {leg_1_ratio_value}, leg 2 ratio: {leg_2_ratio_value}, leg 3 ratio: {leg_3_ratio_value}, leg 4 ratio: {leg_4_ratio_value}."),
            Self::InvalidOccurredAtCrossValue(occurred_at_cross_value) => write!(f, "Message validation error: The specified occurred at cross value '{occurred_at_cross_value}' is not valid."),
            Self::InvalidPrintableValue(printable_value) => write!(f, "Message validation error: The specified printable value '{printable_value}' is not valid."),
            Self::InvalidEventCodeValue(event_code_value) => write!(f, "Message validation error: The specified event code value '{event_code_value}' is not valid."),
            Self::Other(msg) => write!(f, "{msg}")
        }
    }
}

impl Debug for DataFeedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NonISO88591Value(c) => write!(f, "A non-ISO 8859-1 value was detected in alpha field value: {c}."),
            Self::InvalidNumericSize(expected_size) => write!(f, "Attempted to set numeric field of size {expected_size} bytes with value too large."),
            Self::InvalidMessageSize(expected_message_size, actual_message_size) => write!(f, "An invalid message size was provided. Expected: {expected_message_size}, but was: {actual_message_size}."),
            Self::InvalidMessageType(expected_message_type, actual_message_type) => write!(f, "Message validation error: An invalid message type was detected. Expected '{expected_message_type}' but was '{actual_message_type}'."),
            Self::InvalidSideValue(side_value) => write!(f, "Message validation error: The specified side value '{side_value}' is not valid."),
            Self::InvalidExchangeOrderTypeValue(exchange_order_type_value) => write!(f, "Message validation error: The specified exchange order type value '{exchange_order_type_value}' is not valid."),
            Self::InvalidLotTypeValue(lot_type_value) => write!(f, "Message validation error: The specified lot type value '{lot_type_value}' is not valid"),
            Self::InvalidFinancialProductValue(financial_product_value) => write!(f, "Message validation error: The specified financial product value '{financial_product_value}' is not valid"),
            Self::InvalidLegRatioValues(leg_1_ratio_value, leg_2_ratio_value, leg_3_ratio_value, leg_4_ratio_value) => write!(f, "Message validation error: The provided leg ratios do not add to 1. Leg 1 ratio: {leg_1_ratio_value}, leg 2 ratio: {leg_2_ratio_value}, leg 3 ratio: {leg_3_ratio_value}, leg 4 ratio: {leg_4_ratio_value}."),
            Self::InvalidOccurredAtCrossValue(occurred_at_cross_value) => write!(f, "Message validation error: The specified occurred at cross value '{occurred_at_cross_value}' is not valid."),
            Self::InvalidPrintableValue(printable_value) => write!(f, "Message validation error: The specified printable value '{printable_value}' is not valid."),
            Self::InvalidEventCodeValue(event_code_value) => write!(f, "Message validation error: The specified event code value '{event_code_value}' is not valid."),
            Self::Other(msg) => write!(f, "{msg}")
        }
    }
}