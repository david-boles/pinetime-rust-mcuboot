/* Defines memory regions for cortex-m-rt's `link.x.in`. See also `mcuboot.x` */

MEMORY
{
  /* NOTE K = KiBi = 1024 bytes */
  FLASH : ORIGIN = 0x00008020, LENGTH = 464K
  RAM : ORIGIN = 0x20000000, LENGTH = 64K
}


