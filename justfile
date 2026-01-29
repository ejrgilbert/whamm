_default:
    just --list

clean:
    rm -rf progress virgil wizard-engine multi-memory

progress:
    {{justfile_directory()}}/scripts/01_progress.sh

virgil:
    {{justfile_directory()}}/scripts/02_virgil.sh

wizard-engine:
    {{justfile_directory()}}/scripts/03_wizard-engine.sh

multi-memory:
    {{justfile_directory()}}/scripts/04_multi-memory.sh

wham-core:
    {{justfile_directory()}}/scripts/05_wham-core.sh

build:
    {{justfile_directory()}}/scripts/06_build.sh

setup: clean progress virgil wizard-engine multi-memory
