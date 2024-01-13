#[macro_export]
macro_rules! error {
    ($($input:tt)+) => {
        #[cfg(feature = "log")]
        {
            if let Some(log_level) = $crate::log::LOGGER.get() {
                let this = $crate::log::LogLevel::Error;
                if log_level >= &this {
                    println!("{}: {}",this, format_args!($($input)+))
                }
            }
        }
    }
}

#[macro_export]
macro_rules! warn {
    ($($input:tt)+) => {
        #[cfg(feature = "log")]
        {
            if let Some(log_level) = $crate::log::LOGGER.get() {
                let this = $crate::log::LogLevel::Warn;
                if log_level >= &this {
                    println!("{}: {}",this, format_args!($($input)+))
                }
            }
        }
    }
}

#[macro_export]
macro_rules! info {
    ($($input:tt)+) => {
        #[cfg(feature = "log")]
        {

            if let Some(log_level) = $crate::log::LOGGER.get() {
                let this = $crate::log::LogLevel::Info;
                if log_level >= &this {
                    println!("{}: {}",this, format_args!($($input)+))
                }
            }
        }
    }
}

#[macro_export]
macro_rules! debug {
    ($($input:tt)+) => {
        #[cfg(feature = "log")]
        {
            if let Some(log_level) = $crate::log::LOGGER.get() {
                let this = $crate::log::LogLevel::Debug;
                dbg!(&log_level,&this);
                if log_level >= &this {
                    println!("{}: {}",this, format_args!($($input)+))
                }
            }
        }
    }
}

#[macro_export]
macro_rules! trace {
    ($($input:tt)+) => {
        #[cfg(feature = "log")]
        {
            if let Some(log_level) = $crate::log::LOGGER.get() {
                let this = $crate::log::LogLevel::Trace;
                if log_level >= &this {
                    println!("{}: {}",this, format_args!($($input)+))
                }
            }
        }
    }
}
