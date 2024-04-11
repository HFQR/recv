// parse integer type with little endiness.
#[macro_export]
macro_rules! parse_le {
    ($type: ty, $source: expr, $target_len: expr) => {{
        let mut bytes = [0; $target_len];

        debug_assert_eq!($target_len, $source.len());

        let dst = bytes.as_mut_ptr();

        // SAFETY
        //
        // `Slice::copy_from_slice` with length check bypassed.
        //
        // The input source must have the same length as dst. This is only checked
        // at debug mode.
        #[allow(unused_unsafe)]
        unsafe {
            ::core::ptr::copy_nonoverlapping($source.as_ptr(), dst, $target_len);
        }

        <$type>::from_le_bytes(bytes)
    }};
}

// parse 2 bytes to le u16.
#[macro_export]
macro_rules! parse_u16_le {
    ($source: expr) => {
        parse_le!(u16, $source, 2)
    };
}

// parse 4 bytes to le u32.
#[macro_export]
macro_rules! parse_u32_le {
    ($source: expr) => {
        parse_le!(u32, $source, 4)
    };
}

// parse 8 bytes to le u64.
#[macro_export]
macro_rules! parse_u64_le {
    ($source: expr) => {
        parse_le!(u64, $source, 8)
    };
}

// parse 8 bytes to le f64.
#[macro_export]
macro_rules! parse_f64_le {
    ($source: expr) => {
        parse_le!(f64, $source, 8)
    };
}
pub use parse_f64_le;
pub use parse_le;
pub use parse_u16_le;
pub use parse_u32_le;
pub use parse_u64_le;
