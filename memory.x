/*
  Defines memory regions for cortex-m-rt's `link.x.in` and adds dummy sections for mcuboot compatability.
  Since, without overriding the default link script, it's impossible to move the start of the vector table 
  from ORGIN(FLASH), the header is placed in it's own block of memory. See https://github.com/rust-embedded/cortex-m-rt/pull/341.
  See also https://github.com/InfiniTimeOrg/pinetime-mcuboot-bootloader and `imgtool_notes.md`.
*/

MEMORY
{
  /* NOTE K = KiBi = 1024 bytes */
  HEADER : ORIGIN = 0x00008000, LENGTH = 32
  FLASH : ORIGIN = 0x00008020, LENGTH = 475104 /* (464*1024) - 32 */
  RAM : ORIGIN = 0x20000000, LENGTH = 64K
}

SECTIONS {
    .mcuboot_header : {
      FILL(0xAAAAAAAA)
      . = . + 32;
    } > HEADER
}

SECTIONS
{
  .mcuboot_trailer . : {
    FILL(0xFFFFFFFF)
    . = . + 40;
  } > FLASH
} INSERT AFTER .gnu.sgstubs;
