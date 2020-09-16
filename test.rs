use npc;
use wrappers;

fn calculate(world, id) {
  let n = npc::Npc::new();
  let name = n.getname(world, id);
  println(`Hello World {name}`);
  n.setname(world, id, `{name} Bob`);
  let name = n.getname(world, id);
  println(`Hello World {name}`);
  n.getx(world, id) + n.gety(world, id)
}

fn npc_init(npc) {
 2
}

fn npc_speech(npc) {
 2
}

fn npc_move(npc) {
 2
}

fn npc_scene_move(npc) {
 2
}

fn npc_scene_speech(npc) {
 2
}

fn npc_block(npc) {
 2
}

fn npc_drops(npc) {
 2
}

fn npc_skills(npc) {
 2
}

fn npc_quests(npc) {
 3
}

fn npc_shop_items(npc) {
 0
}
