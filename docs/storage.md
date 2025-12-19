# Implementing embedded EEPROM storage for STM32

RMK requires at least two sectors of contiguous flash for its configuration storage feature. Some STM32 chips (like the STM32F401 used in the Keychron Q6 Max) have a strange flash layout that makes this unusually tricky.

| Sector | Address Range             | Size  |
|--------|---------------------------|-------|
| 0      | 0x08000000 - 0x08003FFF   | 16KB  |
| 1      | 0x08004000 - 0x08007FFF   | 16KB  |
| 2      | 0x08008000 - 0x0800BFFF   | 16KB  |
| 3      | 0x0800C000 - 0x0800FFFF   | 16KB  |
| 4      | 0x08010000 - 0x0801FFFF   | 64KB  |
| 5      | 0x08020000 - 0x0803FFFF   | 128KB |

The DFU bootloader burned into the chip needs to boot from the beginning of sector 0 (0x08000000), but you also want plenty of room for the firmware's code. You must keep the first sector avilable for vector table, reserve at least the second and third sectors for storage, and move the rest of the firmware code to the remaining sectors.

Thankfully, the `cortex-m-rt` crate [provides an easy way to do this](https://docs.rs/cortex-m-rt/latest/cortex_m_rt/#_stext) as long as you're willing to roll your own `memory.x` linker script instead of relying on `embassy-stm32`'s `memory-x` feature.

```
MEMORY
{
    FLASH : ORIGIN = 0x08000000, LENGTH = 256K
    RAM   : ORIGIN = 0x20000000, LENGTH = 64K
}

/* The device stores Flash configuration in sectors 1-2 */
/* so place the .text section starting at sector 3 */
_stext = 0x0800C000;
```

Now you can configure RMK to use sectors 1 and 2 for storage.

```rust
let storage_config = StorageConfig {
    // Start at sector 1, 0x4000 from the start of the FLASH region
    start_addr: 0x4000,
    num_sectors: 2,
    ..Default::default()
};
```

There's still a problem, though. As of writing this, RMK and the `sequential-storage` crate it depends on assume sector sizes equal to the `ERASE_SIZE` defined in the provided `Flash` object's `NorFlash` implementation. This is a problem for `embassy-stm32` because it defines `ERASE_SIZE` to be equal to `MAX_ERASE_SIZE` (128KB in the case above) due to the non-uniform sector sizes. To work around this, you can create a wrapper struct that overrides `ERASE_SIZE` to match sectors 1 and 2.

```rust
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
```

Then you can use this wrapper when creating your `Flash` object. Note the usage of `Flash16K`.

```rust
let flash = async_flash_wrapper(Flash16K(Flash::new_blocking(p.FLASH)));
```
