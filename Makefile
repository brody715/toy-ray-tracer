ARGS=

export RAYON_NUM_THREADS=6

rt-build-optimized:
	cargo build -p toy_ray_tracer --release

SCENE=earth
rt-run-optimized: rt-build-optimized
	time ./target/release/toy_ray_tracer --scene=${SCENE} ${ARGS}
