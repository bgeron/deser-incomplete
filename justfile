
_list:
    just --list

rebuild-all: rebuild-vhs rebuild-html

rebuild-vhs:
    just -f ~/src/partial-deser/examples/vhs/justfile build-all
    rm -f assets/*.gif
    mv ~/src/partial-deser/examples/vhs/target/*.gif ~/src/partial-deser-doc/assets/

rebuild-html:
    just -f ~/src/partial-deser/justfile doc-public
    rm -rf html
    mv ~/src/partial-deser/target-nightly/doc html