#!/bin/bash

(

    printf "Testing GitHub output ... "

    cd "$(dirname "${BASH_SOURCE[0]}")"

    git remote rm origin
    git remote add origin git@github.com:neogeek/generate-local-changelog.git

    if [ "${GENERATE_FIXTURES}" ]; then

        ../bin/generate-local-changelog >./fixtures/github.md

    fi

    ../bin/generate-local-changelog | diff ./fixtures/github.md -

    printf "PASSED\n"

)
