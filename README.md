Inu Config Flash Tool
=====================
This tool creates binary files for the settings to flash to Inu Ferric-edition devices.

It takes a JSON settings input, and will create a binary file that can be used by `espflash` to write to the
settings partition of the target device.

**Usage:**

    inu-cfg-flash -i <input.json> -o <output.bin>


Installing
----------

    cargo install --path .
