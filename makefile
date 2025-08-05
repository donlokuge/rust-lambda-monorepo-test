APP_NAME=node-lambda
DIST_DIR=dist/apps/$(APP_NAME)
ZIP_FILE=$(APP_NAME).zip

.PHONY: all build zip clean

all: clean build zip

build:
	nx build $(APP_NAME)

zip: build
	cd $(DIST_DIR) && zip -r ../../$(ZIP_FILE) .

clean:
	rm -f dist/$(ZIP_FILE)
	rm -rf $(DIST_DIR)
