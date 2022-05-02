import {default_block_size, make_assignment2} from "./base";

export default make_assignment2({
  name: "assignment2-random_fixed",
  sampler: {
    kind: "random_fixed",
    block_size: default_block_size,
  },
});
