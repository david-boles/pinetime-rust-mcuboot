/* Linker script that runs after cortex-m-rt's `link.x.in`. This is important for knowing where to place the trailer. */

SECTIONS {
    .mcuboot_header ORIGIN(FLASH):
    {
        FILL(0xAAAAAAAA)
        . = . + 32;
    } > FLASH

    .mcuboot_trailer __veneer_limit :
    {
        FILL(0xFFFFFFFF)
        . = . + 40;
    } > FLASH
}