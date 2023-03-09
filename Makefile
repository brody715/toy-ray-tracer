ifndef VERSION_TAG
COMMIT_ID = $(shell git log -1 --format=%h)
VERSION_TAG = ${COMMIT_ID}
endif

DOCKER_REGISTRY = registry.cn-shanghai.aliyuncs.com/brody715/

EXEC_IMAGE_NOTAG = ${DOCKER_REGISTRY}toy-ray-tracer
EXEC_IMAGE = ${EXEC_IMAGE_NOTAG}:${VERSION_TAG}

GARGS =
ARGS=

export RAYON_NUM_THREADS=6

build-exec-image:
	docker buildx build . \
      -t ${EXEC_IMAGE_NOTAG}:v1 \
      -t ${EXEC_IMAGE} \

SCENE=homework1
run-exec-image:
	time docker run -v $(shell pwd)/output:/app/output ${EXEC_IMAGE} ${GARGS} render --project-file=assets/projects/${SCENE}.js ${ARGS}

push-exec-image:
	docker push ${EXEC_IMAGE}

print-exec-image:
	@echo ${EXEC_IMAGE}

RELEASE_EXE = ./target/release/toy_ray_tracer

rt-build-optimized:
	cargo build -p toy_ray_tracer --release

SCENE=earth
rt-run-render: rt-build-optimized
	RUST_BACKTRACE=1 time ${RELEASE_EXE} ${GARGS} render --project-file=./assets/projects/${SCENE}.js ${ARGS}

rt-show-scene:
	code ./output/${SCENE}.png

rt-generate-jsonschema: rt-build-optimized
	${RELEASE_EXE} ${GARGS} generate ${ARGS} > assets/schemas/project.json
	json2ts -i assets/schemas/project.json -o assets/schemas/project.d.ts --strictIndexSignatures
