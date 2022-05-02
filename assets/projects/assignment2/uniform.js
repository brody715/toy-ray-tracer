import {default_block_size, make_assignment2} from "./base";

export default make_assignment2({
  name: "assignment2-uniform",
  sampler: {
    kind: "uniform",
    block_size: default_block_size,
  },
});
