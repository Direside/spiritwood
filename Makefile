GIT_HEAD = $(shell git rev-parse HEAD | cut -b -7)
DOCKER_IMG=spiritwood-server

image: release
	docker build . -t $(DOCKER_IMG):$(GIT_HEAD)
	docker tag $(DOCKER_IMG):$(GIT_HEAD) $(DOCKER_IMG):latest

tag:
	docker tag $(DOCKER_IMG):$(GIT_HEAD) $(DOCKER_REPO)/$(DOCKER_IMG):$(GIT_HEAD)
	docker tag $(DOCKER_IMG):$(GIT_HEAD) $(DOCKER_REPO)/$(DOCKER_IMG):latest

release:
	GIT_HEAD=$(GIT_HEAD) cargo build --release

push: image tag
	docker push $(DOCKER_REPO)/$(DOCKER_IMG):$(GIT_HEAD)
	docker push $(DOCKER_REPO)/$(DOCKER_IMG):latest

run:
	docker run $(DOCKER_IMG):latest

deploy: login image tag push

login:
	$$(aws ecr get-login --no-include-email)
