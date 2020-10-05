#!/bin/bash

(

    printf "Testing Bitbucket output ... "

    cd "$(dirname "${BASH_SOURCE[0]}")"

    git remote rm origin
    git remote add origin git@bitbucket.com:neogeek/generate-local-changelog.git

    if [ "${GENERATE_FIXTURES}" ]; then

        ../bin/generate-local-changelog >./fixtures/bitbucket.md

    fi

    ../bin/generate-local-changelog | diff ./fixtures/bitbucket.md -

    printf "PASSED\n"

)
