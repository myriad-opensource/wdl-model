.PHONY: clean build test package lint format
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
