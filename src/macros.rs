#[macro_export]
macro_rules! unwrap_or_continue_r {
    // Continue — with message
    ($expr:expr, $($msg:tt)+) => {
        match $expr {
            Ok(value) => value,
            Err(err) => {
                tracing::error!($($msg)+, err);
                continue;
            }
        }
    };

    // Continue — no message
    ($expr:expr) => {
        match $expr {
            Ok(value) => value,
            Err(err) => {
                tracing::error!("operation failed: {}", err);
                continue;
            }
        }
    };
}

#[macro_export]
macro_rules! unwrap_or_continue_o {
    ($expr:expr, $($msg:tt)+) => {
        match $expr {
            Some(value) => value,
            None => {
                tracing::error!($($msg)+);
                continue;
            }
        }
    };

    ($expr:expr) => {
        match $expr {
            Some(value) => value,
            None => {
                tracing::error!("operation failed: got None");
                continue;
            }
        }
    };
}
