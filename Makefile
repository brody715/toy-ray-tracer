GARGS =
ARGS=

export RAYON_NUM_THREADS=6

RELEASE_EXE = ./target/release/toy_ray_tracer

rt-build-optimized:
	cargo build -p toy_ray_tracer --release

rt-run-optimized: rt-build-optimized
	time ${RELEASE_EXE} ${GARGS} ${ARGS}

SCENE=earth
rt-run-render: rt-build-optimized
	time ${RELEASE_EXE} ${GARGS} render --project-file=./assets/projects/${SCENE}.js ${ARGS}

rt-show-scene:
	code ./output/${SCENE}.png

rt-generate-jsonschema: rt-build-optimized
	${RELEASE_EXE} ${GARGS} generate ${ARGS} > assets/schemas/project.json
	json2ts -i assets/schemas/project.json -o assets/schemas/project.d.ts --strictIndexSignatures
