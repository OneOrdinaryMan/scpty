CC=cargo
CCFLAGS=
BUILDER_DEBUG=$(CC) build
BUILDER_RELEASE=$(CC) build --release
TARGET_DEBUG_DIR=target/debug
TARGET_RELEASE_DIR=target/release
TARGET=scpty
SRC_FILES=$(wildcard src/*.rs)
RESOURCE_DIR=resources
SHARE_FILES=$(patsubst $(RESOURCE_DIR)/%, $(SHARE_DIR)/%.tar.gz, $(wildcard $(RESOURCE_DIR)/*))
SHARE_DIR=share
BIN_LOCATION=/usr/bin
SHARE_LOCATION=/usr/share/scpty
.DEFAULT_GOAL:=all
.PHONY: clean remake debug release all all_r
prepare: $(SHARE_FILES)
$(SHARE_DIR):
	mkdir $@
$(SHARE_DIR)/%.tar.gz: $(RESOURCE_DIR)/%/ $(SHARE_DIR)
	@echo "Creating the tarball $@"
	@tar -czf $@ -C $< .
debug: $(TARGET_DEBUG_DIR)/$(TARGET)
$(TARGET_DEBUG_DIR)/$(TARGET): $(SRC_FILES)
	$(CC) check
	$(BUILDER_DEBUG)
release: $(TARGET_RELEASE_DIR)/$(TARGET)
$(TARGET_RELEASE_DIR)/$(TARGET): $(SRC_FILES)
	$(BUILDER_RELEASE)
clean:
	@echo "Cleaning the target and removing the assets."
	@$(CC) clean
	@rm -r $(SHARE_DIR)/*
remake: clean prepare debug
all: prepare debug
all_r: prepare release
$(SHARE_LOCATION):
	@echo "The directory not found. Making the share directory."
	@mkdir $@
install: $(TARGET_RELEASE_DIR)/$(TARGET) $(SHARE_FILES) $(SHARE_LOCATION)
	@echo "Transferring the binary."
	@cp $< $(BIN_LOCATION)
	@echo "Cleaning the asset dir for new files."
	@touch $(SHARE_LOCATION)/testfile
	@rm -r $(SHARE_LOCATION)/*
	@echo "Transferring the assets to the share directory."
	@cp $(SHARE_FILES) $(SHARE_LOCATION)/
uninstall: $(BIN_LOCATION)/$(TARGET) $(SHARE_LOCATION)
	@echo "Removing the binary."
	@rm $<
	@echo "Removing the assets."
	@rm -r $(SHARE_LOCATION)
print:
	@echo "$(SHARE_FILES)"
