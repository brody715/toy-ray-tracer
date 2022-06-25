import {make_mis_scene, smooth_material} from "./mis-base";

export default make_mis_scene({
  name: "mis-smooth-brdf",
  mis_weight: 1.0,
  board_material: smooth_material,
});
