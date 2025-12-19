use embedded_storage::nor_flash::{ErrorType, NorFlash, ReadNorFlash};

/// Wrapper to report 16KB erase size for STM32F4 sectors 1-2
pub struct Flash16K<F>(pub F);

impl<F: ErrorType> ErrorType for Flash16K<F> {
    type Error = F::Error;
}

impl<F: ReadNorFlash> ReadNorFlash for Flash16K<F> {
    const READ_SIZE: usize = F::READ_SIZE;

    fn read(&mut self, offset: u32, bytes: &mut [u8]) -> Result<(), Self::Error> {
        self.0.read(offset, bytes)
    }

    fn capacity(&self) -> usize {
        self.0.capacity()
    }
}

impl<F: NorFlash> NorFlash for Flash16K<F> {
    const WRITE_SIZE: usize = F::WRITE_SIZE;
    const ERASE_SIZE: usize = 16 * 1024; // 16KB for sectors 1-2

    fn erase(&mut self, from: u32, to: u32) -> Result<(), Self::Error> {
        self.0.erase(from, to)
    }

    fn write(&mut self, offset: u32, bytes: &[u8]) -> Result<(), Self::Error> {
        self.0.write(offset, bytes)
    }
}
