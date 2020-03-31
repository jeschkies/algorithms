.DEFAULT_GOAL := all

.PHONY: all
all: lib/ksum.log

lib/%.log:
	python lib/$*.py | tee $@.work
	@mv $@.work $@

.PHONY: clean
clean:
	rm lib/*.log
