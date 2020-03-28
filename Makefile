GIT_HEAD = $(shell git rev-parse HEAD | cut -b -7)

release:
	GIT_HEAD=$(GIT_HEAD) cargo build --release

push:
	ssh $(RHOST) killall spiritwood-server; \
	scp target/release/spiritwood-server $(RHOST):
	ssh $(RHOST) 'nohup ./spiritwood-server &'
