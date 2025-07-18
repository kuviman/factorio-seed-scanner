help:
    echo Look in .justfile

run *ARGS:
    FACTORIO=~/factorio/experimental/bin/x64/factorio steam-run cargo run --release -- {{ARGS}}