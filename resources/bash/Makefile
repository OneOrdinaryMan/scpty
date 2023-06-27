SRC_DIR=src
SRC_FILE=$(SRC_DIR)/script.bash
TARGET=hello_world
TARGET_DIR=target
TARGET_FILE=$(TARGET_DIR)/$(TARGET)
BIN_LOCATION=/usr/bin/

.DEFAULT_GOAL:=build
.PHONY: clean remake build

$(TARGET_DIR):
	@echo "Building the Directory"
	@mkdir $@

build: $(TARGET_FILE)

$(TARGET_FILE): $(SRC_FILE) $(TARGET_DIR)
	@echo "Building the binary"
	@cp -r $< $@
	@echo "Setting correct permissions"
	@chmod 751 $@
	
run: $(TARGET_FILE)
	@./$<

clean:
	@{\
		if [ -d $TARGET_DIR ]; then \
			echo "Cleaning the target directory"; \
			rm -r $<; \
		fi; \
	}

remake: clean build

install: $(TARGET_FILE) $(BIN_LOCATION)
	@echo "Installing the components"
	@cp $< $(BIN_LOCATION)

uninstall: $(BIN_LOCATION)/$(TARGET)
	@echo "Uninstalling the program"
	@rm $<
