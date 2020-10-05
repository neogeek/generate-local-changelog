#!/bin/bash

(

    printf "Testing GitLab output ... "

    cd "$(dirname "${BASH_SOURCE[0]}")"

    git remote rm origin
    git remote add origin git@gitlab.com:neogeek/generate-local-changelog.git

    if [ "${GENERATE_FIXTURES}" ]; then

        ../bin/generate-local-changelog >./fixtures/gitlab.md

    fi

    ../bin/generate-local-changelog | diff ./fixtures/gitlab.md -

    printf "PASSED\n"

)
