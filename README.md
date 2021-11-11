Rust gallery scanner
====


About
----

Image deduplicate script.

[![rustc 1.55.0](https://img.shields.io/badge/rustc-v1.55.0-green)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-BSD-blue.svg)](https://raw.githubusercontent.com/manti-by/mlibrary/master/LICENSE)

Author: Alexander Chaika <manti.by@gmail.com>

Source link: https://github.com/manti-by/mgallery-rust/

Python mirror: https://github.com/manti-by/mgallery-py/

Requirements:

    RustC 1.55.0, Image 0.23.14, ImageHash 3.2


Script setup
----

1. Create log dir and db path

        $ make setup

2. Scan gallery

        $ make scan


NOTE: For additional commands check Makefile