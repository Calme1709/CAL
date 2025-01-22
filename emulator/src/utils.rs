// FIXME: There is probably a non-iterative way to do this
#[macro_export]
macro_rules! decode_signed_integer {
    ( $encoded:expr, $bits:expr ) => {{
        (if $encoded & 1 << ($bits - 1) != 0 { (((2_u16).pow(16 - $bits) - 1) << $bits) } else { 0 } | $encoded) as i16
    }}
}