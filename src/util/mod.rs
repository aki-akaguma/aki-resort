pub use self::opt_according_to_word::OptAccordingToWord;
pub use self::opt_according_to_word::OptAccordingToWordParseError;
pub use self::opt_color_when::OptColorWhen;
pub use self::opt_color_when::OptColorWhenParseError;
pub use self::opt_max_buffer_size::OptMaxBufferSize;
pub use self::opt_max_buffer_size::OptMaxBufferSizeParseError;

pub mod err;
mod opt_according_to_word;
mod opt_color_when;
mod opt_max_buffer_size;
