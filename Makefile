CURRENT_DIR = $(shell pwd)

define MIGRATION_SCRIPT
CREATE TABLE IF NOT EXISTS gallery (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    path TEXT,
    name TEXT,
    datetime DATETIME DEFAULT CURRENT_TIMESTAMP
);
CREATE TABLE IF NOT EXISTS image (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    gallery_id INTEGER,
    path TEXT,
    name TEXT,
    phash TEXT,
    size INTEGER,
    width INTEGER,
    height INTEGER,
    datetime DATETIME DEFAULT CURRENT_TIMESTAMP
);
endef

export CURRENT_DIR
export MIGRATION_SCRIPT
setup:
	sqlite3 $$CURRENT_DIR/db.sqlite "$$MIGRATION_SCRIPT" && \
	mkdir -p $$CURRENT_DIR/log/ && \
	touch $$CURRENT_DIR/log/debug.log && \
	touch $$CURRENT_DIR/log/error.log

export CURRENT_DIR
clean:
	rm -rf $$CURRENT_DIR/db.sqlite && \
	rm -rf $$CURRENT_DIR/log/

scan:
	./target/debug/mlibrary
