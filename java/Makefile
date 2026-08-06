PROFILE_DIR ?= .profiles
PROFILE_ITERATIONS ?= 2000
PROFILE_WARMUP ?= 100
RELEASE_BRANCH ?= main

.PHONY: clean build test package publish-prerelease publish-release lint format profile-mem profile-mem-ui
clean:
	@mvn clean

build:
	@mvn clean compile

test:
	@mvn test

package:
	@mvn clean package

publish-prerelease:
	@VERSION=$$(mvn -q -DforceStdout help:evaluate -Dexpression=project.version); \
	if ! echo "$$VERSION" | grep -q -- '-SNAPSHOT$$'; then \
		echo "ERROR: publish-prerelease requires a -SNAPSHOT version in pom.xml, got $$VERSION"; \
		exit 1; \
	fi; \
	echo "Publishing Maven snapshot $$VERSION to GitHub Packages..."; \
	mvn deploy

publish-release:
	@VERSION=$$(mvn -q -DforceStdout help:evaluate -Dexpression=project.version); \
	if echo "$$VERSION" | grep -q -- '-SNAPSHOT$$'; then \
		echo "ERROR: publish-release requires a non-SNAPSHOT version in pom.xml, got $$VERSION"; \
		exit 1; \
	fi; \
	BRANCH=$$(git rev-parse --abbrev-ref HEAD); \
	if [ "$$BRANCH" != "$(RELEASE_BRANCH)" ]; then \
		echo "ERROR: releases must be cut from '$(RELEASE_BRANCH)'; current branch is '$$BRANCH'"; \
		exit 1; \
	fi; \
	if ! command -v gh >/dev/null 2>&1; then \
		echo "ERROR: GitHub CLI (gh) is required to cut a release."; \
		exit 1; \
	fi; \
	TAG_NAME=java-$$VERSION; \
	if git rev-parse "$$TAG_NAME" >/dev/null 2>&1 || git ls-remote --tags origin "refs/tags/$$TAG_NAME" | grep -q .; then \
		echo "ERROR: tag $$TAG_NAME already exists. Bump version in pom.xml before releasing again."; \
		exit 1; \
	fi; \
	echo "Publishing Maven release $$VERSION to GitHub Packages..."; \
	mvn deploy; \
	echo "Creating annotated tag $$TAG_NAME and GitHub release..."; \
	git tag -a "$$TAG_NAME" -m "$$TAG_NAME"; \
	git push origin "$$TAG_NAME"; \
	gh release create "$$TAG_NAME" --title "$$TAG_NAME" --generate-notes

lint:
	@mvn spotless:check

format:
	@mvn spotless:apply

profile-mem:
	@mkdir -p $(PROFILE_DIR)
	@MAVEN_OPTS="-XX:StartFlightRecording=filename=$(PROFILE_DIR)/profile.jfr,settings=profile,dumponexit=true" \
		mvn -q clean -DskipTests \
		-Dexec.mainClass=org.openwdl.wdl.model.benchmarks.WdlBenchmarkRunner \
		-Dexec.classpathScope=test \
		-Dexec.args="--iterations $(PROFILE_ITERATIONS) --warmup $(PROFILE_WARMUP) --bench-output $(PROFILE_DIR)/bench.txt" \
		test-compile org.codehaus.mojo:exec-maven-plugin:3.5.0:java
	@echo "Wrote Java benchmark/profile artifacts to $(PROFILE_DIR)/"

profile-mem-ui:
	@if [ ! -f $(PROFILE_DIR)/profile.jfr ] || [ ! -f $(PROFILE_DIR)/bench.txt ]; then $(MAKE) profile-mem; fi
	@echo "Open JFR in Java Mission Control or another JFR viewer: $(PROFILE_DIR)/profile.jfr"
	@echo "Benchmark summary: $(PROFILE_DIR)/bench.txt"
	@if command -v xdg-open >/dev/null 2>&1; then xdg-open $(PROFILE_DIR); \
	elif command -v open >/dev/null 2>&1; then open $(PROFILE_DIR); \
	elif command -v explorer.exe >/dev/null 2>&1; then explorer.exe $(PROFILE_DIR); \
	else echo "Open the profiles directory manually if needed."; fi
