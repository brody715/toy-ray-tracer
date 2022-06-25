import {make_mis_scene, rough_material} from "./mis-base";

export default make_mis_scene({
  name: "mis-rough-mixed",
  mis_weight: 0.5,
  board_material: rough_material,
});
