cd mp4box
cargo build --release

# Print the box tree
target/release/mp4dump path/to/video.mp4

# Decode known boxes when available (example includes ftyp)
target/release/mp4dump path/to/video.mp4 --decode

# Dump raw payload bytes of a given box (first match at all depths)
target/release/mp4dump path/to/video.mp4 --raw stsd --bytes 256

# Dump a UUID box payload (prefix match ok)
target/release/mp4dump path/to/file.mp4 --raw uuid:00000000-...   # or compact hex prefix after 'uuid:'
