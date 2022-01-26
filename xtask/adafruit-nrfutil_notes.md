manifest.json:

```
{
    "manifest": {
        "application": {
            "bin_file": "image.bin",
            "dat_file": "image.dat",
            "init_packet_data": {
                "application_version": 4294967295,
                "device_revision": 65535,
                "device_type": 65535,
                "firmware_crc16": 61399,
                "softdevice_req": [
                    65534
                ]
            }
        },
        "dfu_version": 0.5
    }
}
```

```
image.dat, all numbers are unsigned, little endian:
Offset: 00 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F
        ff ff : DEVICE TYPE
              ff ff : DEVICE REVISION
                    ff ff ff ff : APP VERSION
                                01 00 : # OF SOFTDEVICE REQUIREMENTS
                                      fe ff : SOFTDEVICE REQUIREMENT
                                            d7 ef : CRC16
```