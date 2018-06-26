.PHONY: all openocd debug

OPENOCD := openocd
OPENOCD_CFG := utils/stm32h7x3i.cfg
BIN := target/thumbv7em-none-eabihf/release/app

# Build and flash in release mode
all:
	cargo build --release
	$(OPENOCD) -f $(OPENOCD_CFG) -c "program $(BIN) reset exit"

# Start a openocd session.
openocd:
	$(OPENOCD) -f $(OPENOCD_CFG)

# Start a gdb session. Works if a valid openocd session is existing.
debug:
	arm-none-eabi-gdb $(BIN) -q -x utils/debug.gdb
