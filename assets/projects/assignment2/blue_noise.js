import {make_assignment2, default_block_size} from "./base";

export default make_assignment2({
  name: "assignment2-blue_noise",
  sampler: {
    kind: "blue_noise",
    block_size: default_block_size,
  },
});
