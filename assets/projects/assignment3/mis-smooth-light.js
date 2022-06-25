import {make_mis_scene, smooth_material} from "./mis-base";

export default make_mis_scene({
  name: "mis-smooth-light",
  mis_weight: 0.0,
  board_material: smooth_material,
});
