#!/bin/bash
# Run both cargo watch and tailwind compiler in parallel
cargo watch -x run & ./tailwindcss -i input.css -o templates/output.css --watch
