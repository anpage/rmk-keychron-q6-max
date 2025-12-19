/* memory.x for STM32F401xC */

MEMORY
{
    FLASH : ORIGIN = 0x08000000, LENGTH = 256K
    RAM   : ORIGIN = 0x20000000, LENGTH = 64K
}

/* The device stores Flash configuration in sectors 1-2, so we place .text after that */
_stext = 0x0800C000;
