Results of `cargo build` and `rust-objcopy -O binary ...` are in the target folder.

Flash error:
```
$ cargo flash --elf target/thumbv7em-none-eabihf/debug/firmware --chip nRF52832_xxAA
    Flashing target/thumbv7em-none-eabihf/debug/firmware
       Error 0: Error while flashing
             1: Adding data for addresses 00008000..0000B668 overlaps previously added data for addresses 00008020..0000AE24.
```

Segments:
```
$ readelf -l target/thumbv7em-none-eabihf/debug/firmware

Elf file type is EXEC (Executable file)
Entry point 0x80fd
There are 8 program headers, starting at offset 52

Program Headers:
  Type           Offset   VirtAddr   PhysAddr   FileSiz MemSiz  Flg Align
  PHDR           0x000034 0x00000034 0x00000034 0x00140 0x00140 R   0x4
  LOAD           0x000000 0x00000000 0x00000000 0x00174 0x00174 R   0x10000
  LOAD           0x008020 0x00008020 0x00008020 0x000dc 0x000dc R   0x10000
  LOAD           0x0080fc 0x000080fc 0x000080fc 0x02d28 0x02d28 R E 0x10000
  LOAD           0x00ae30 0x0000ae30 0x0000ae30 0x007f8 0x007f8 R   0x10000
  LOAD           0x010000 0x20000000 0x20000000 0x00000 0x00444 RW  0x10000
  LOAD           0x018000 0x00008000 0x00008000 0x03668 0x03668 RW  0x10000
  GNU_STACK      0x000000 0x00000000 0x00000000 0x00000 0x00000 RW  0

 Section to Segment mapping:
  Segment Sections...
   00     
   01     
   02     .vector_table 
   03     .text 
   04     .rodata 
   05     .bss 
   06     .mcuboot_header .mcuboot_trailer 
   07
```

Sections:
```
$ rust-size -A target/thumbv7em-none-eabihf/debug/firmware
target/thumbv7em-none-eabihf/debug/firmware  :
section                size        addr
.vector_table           220       32800
.text                 11560       33020
.rodata                2040       44592
.data                     0   536870912
.gnu.sgstubs              0       46656
.bss                   1092   536870912
.uninit                   0   536872004
.mcuboot_header          32       32768
.mcuboot_trailer         40       46656
.debug_abbrev         32671           0
.debug_info          642704           0
.debug_aranges        35544           0
.debug_str           710689           0
.debug_pubnames      171084           0
.debug_pubtypes      150018           0
.ARM.attributes          58           0
.debug_frame         106268           0
.debug_line          364939           0
.debug_ranges        162352           0
.debug_loc             1885           0
.debug_line_str          34           0
.debug_rnglists          25           0
.comment                109           0
Total               2393364
```