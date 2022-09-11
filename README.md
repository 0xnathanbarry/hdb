# Hex Decimal Binary CLI

- Can easily convert between Hex Decimal and Binary numbers
- Can get the Hex number of a binary mask

## Install Instructions
- git clone <URL>
- run `cargo run --path .`

## Commands
- `hdb --hb <HEX_NUMBER>` to convert Hexadecimal to Binary
- `hdb --hd <HEX_NUMBER>` to convert Hexadecimal to Decimal
- `hdb --bh <BINARY_NUMBER>` to convert Binary to Hexadecimal
- `hdb --bd <BINARY_NUMBER>` to convert Binary to Decimal
- `hdb --dh <NUMBER>` to convert Decimal to Hexadecimal
- `hdb --db <NUMBER>` to convert Decimal to Binary
- `hdb --mask <NUM_OF_ONES>,<NUM_OF_ZEROS>` to create a mask
	```
	// Ex for mask
	$ hdb --mask 3,5
	Binary Mask: 11100000
	Decimal: 224
	Hex: E0
	```
	
## Notes
- Currently no support for unsigned decimal integers
- For Hex, must provide an even amount of characters (0F11, not F11, pad right with a 0)

## TODO
- [ ] Figure out how to dynamically pad binary with zeros
- [ ] Create format binary function to print binaries with spaces between every byte
- [ ] Create solution for unsigned integers
- [ ] Add padding 0 for hex numbers if insert odd number of characters
- [X] Create Masking Function
- [X] Create Hex conversion
- [X] Create Decimal conversion
- [X] Create Binary conversion
