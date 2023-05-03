request:
	@echo "Usage: make request"
	sh requests.sh $(p) $(m)

test:
	@echo "Usage: make test"
	sh test.sh
	rm default_*.profraw

clean:
	@echo "Usage: make clean"
	rm default_*.profraw
	rm -rf target/debug/coverage
