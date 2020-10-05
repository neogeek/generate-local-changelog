test:
	shellcheck ./bin/generate-local-changelog
	@test/bitbucket.sh
	@test/github.sh
	@test/gitlab.sh
	@(git remote rm origin && git remote add origin git@github.com:neogeek/generate-local-changelog.git)

changelog:
	./bin/generate-local-changelog > CHANGELOG.md

.PHONY: test
