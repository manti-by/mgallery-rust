CURRENT_DIR = $(shell pwd)

define MIGRATION_SCRIPT
CREATE TABLE IF NOT EXISTS image (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    path TEXT,
    name TEXT,
    phash TEXT,
    size INTEGER,
    width INTEGER,
    height INTEGER,
    datetime DATETIME DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX idx_image_path ON image (path);
CREATE INDEX idx_image_phash ON image (phash);
endef

export CURRENT_DIR
export MIGRATION_SCRIPT
setup:
	sqlite3 $$CURRENT_DIR/db.sqlite "$$MIGRATION_SCRIPT" && \
	mkdir -p $$CURRENT_DIR/logs/ && \
	touch $$CURRENT_DIR/logs/debug.log && \
	touch $$CURRENT_DIR/logs/error.log

export CURRENT_DIR
clean:
	rm -rf $$CURRENT_DIR/db.sqlite && \
	rm -rf $$CURRENT_DIR/logs/

debug:
	cargo build

release:
	cargo build --release

scan:
	./target/release/mgallery --data_path $$CURRENT_DIR/../data/ --db_path $$CURRENT_DIR/db.sqlite

reload: clean setup release scan
