pub type BasicResult = Result<(), BasicError>;

pub struct BasicError {
    pub message: &'static str,
}

impl BasicError {
    pub fn from_usart_error(error: embassy_stm32::usart::Error) -> Self {
        BasicError {
            message: match error {
                embassy_stm32::usart::Error::Framing => "USART error: Framing",
                embassy_stm32::usart::Error::Noise => "USART error: Noise",
                embassy_stm32::usart::Error::Overrun => "USART error: Overrun",
                embassy_stm32::usart::Error::Parity => "USART error: Parity",
                embassy_stm32::usart::Error::BufferTooLong => "USART error: Buffer too long",
                _ => "USART error: Unknown",
            },
        }
    }
}
