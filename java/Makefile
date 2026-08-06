PROFILE_DIR ?= .profiles
PROFILE_ITERATIONS ?= 2000
PROFILE_WARMUP ?= 100

.PHONY: clean build test package lint format profile-mem profile-mem-ui
clean:
	@mvn clean

build:
	@mvn clean compile

test:
	@mvn test

package:
	@mvn clean package

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
