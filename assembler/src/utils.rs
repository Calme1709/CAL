#[macro_export]
macro_rules! encode_signed_integer {
    ( $integer:expr, $bits:expr, $span:expr ) => {{
        use crate::assembler::AssemblerError;

        let min_value = -(2 as i32).pow($bits - 1);
        let max_value = (2 as i32).pow($bits - 1) - 1;

        match $integer >= min_value && $integer <= max_value {
            true => Ok($integer as u16 & ((2 as u16).pow($bits) - 1)),
            false => Err(AssemblerError { span: $span, error: format!("Invalid value for i{} \"{}\", values should be in range {}-{}", $bits, $integer, min_value, max_value) })
        }
    }}
}

#[macro_export]
macro_rules! encode_unsigned_integer {
    ( $integer:expr, $bits:expr, $span:expr ) => {{
        use crate::assembler::AssemblerError;

        let min_value = 0;
        let max_value = (2 as i32).pow($bits) - 1;

        match $integer >= min_value && $integer <= max_value {
            true => Ok($integer as u16 & ((2 as u16).pow($bits) - 1)),
            false => Err(AssemblerError { span: $span, error: format!("Invalid value for u{} \"{}\", values should be in range 0-{}", $bits, $integer, max_value) })
        }
    }}
}
