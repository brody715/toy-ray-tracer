import {make_mis_scene, rough_material, smooth_material} from "./mis-base";

export default make_mis_scene({
  name: "mis-rough-light",
  mis_weight: 0.0,
  board_material: rough_material,
});
