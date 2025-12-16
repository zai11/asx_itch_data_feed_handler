use crate::{enums::data_feed_error::DataFeedError, models::data_types::{alpha::Alpha, numeric::{self, Numeric}, price::Price}};

#[inline(always)]
pub fn write_num<const SIZE: usize>(
    buf: &mut [u8],
    offset: usize,
    bytes: &[u8; SIZE]
) {
    buf[offset..offset + SIZE].copy_from_slice(bytes);
}

#[inline(always)]
pub fn write_alpha<const SIZE: usize>(
    buf: &mut [u8],
    offset: usize,
    chars: &[char; SIZE]
) {
    let dst = &mut buf[offset..offset + SIZE];
    for i in 0..SIZE {
        dst[i] = chars[i] as u8;
    }
}

#[inline(always)]
pub fn read_numeric<const SIZE: usize>(
    buf: &[u8],
    offset: usize,
) -> Result<Numeric<SIZE>, DataFeedError> {
    if !numeric::VALID_NUMERIC_SIZES.contains(&SIZE) {
        return Err(DataFeedError::InvalidNumericSize(SIZE));
    }

    let src = &buf[offset..offset + SIZE];

    let mut value: u128 = 0;
    for &b in src {
        value = (value << 8) | b as u128;
    }

    Ok(Numeric { value })
}

#[inline(always)]
pub fn read_price(
    buf: &[u8],
    offset: usize,
) -> Price {
    let bytes = [
        buf[offset],
        buf[offset + 1],
        buf[offset + 2],
        buf[offset + 3],
    ];

    let value = i32::from_be_bytes(bytes);

    Price { value }
}

#[inline(always)]
pub fn read_alpha<const SIZE: usize>(
    buf: &[u8],
    offset: usize,
) -> Result<Alpha<SIZE>, DataFeedError> {
    let mut char_arr = [' '; SIZE];
    let src = &buf[offset..offset + SIZE];

    for i in 0..SIZE {
        let b = src[i];

        if b < 0x20 || b > 0x7F {
            return Err(DataFeedError::NonISO88591Value(b as char));
        }

        char_arr[i] = b as char;
    }

    Ok(
        Alpha {
            value: char_arr
        }
    )
}