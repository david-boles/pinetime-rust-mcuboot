/* Linker script that runs after cortex-m-rt's `link.x.in`. This is important for knowing where to place the trailer. */

SECTIONS {
    .mcuboot_header 0x8000 :
    {
        FILL(0xAAAAAAAA)
        . = . + 32;
    } > FLASH

    .mcuboot_trailer __erodata :
    {
        FILL(0xFFFFFFFF)
        . = . + 40;
    } > FLASH
}