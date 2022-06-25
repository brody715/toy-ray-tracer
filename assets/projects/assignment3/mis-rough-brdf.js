import {make_mis_scene, rough_material} from "./mis-base";

export default make_mis_scene({
  name: "mis-rough-brdf",
  mis_weight: 1.0,
  board_material: rough_material,
});
