macro_rules! concat_parts {
    ($parts_head:expr) => {
        // `stringify!` will convert the expression *as it is* into a string.
        format!(
            "{}{}",
            $crate::models::service::transactions::ID_SEPARATOR,
            $parts_head
        );
    };
    ($parts_head:expr, $($parts_tail:expr),+) => {
        // `stringify!` will convert the expression *as it is* into a string.
        format!(
            "{}{}{}",
            $crate::models::service::transactions::ID_SEPARATOR,
            $parts_head,
            concat_parts!($($parts_tail),+)
        );
    };
}

macro_rules! create_id {
    ($tx_type:expr, $($parts:expr),+) => {
        // `stringify!` will convert the expression *as it is* into a string.
        format!("{}{}", $tx_type, concat_parts!($($parts),+));
    };
}