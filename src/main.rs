mod npc;
mod wrappers;

use runestick::FromValue as _;
use wrappers::*;
use npc::*;

fn main() {
  let mut world = LegionWrap::new();

  let npcid = world.inner.push((Npc, Kind::Npc, Position { x: 0, y: 0, map: 1 }, Hp(1), Name("Bob".to_string())));
  world.inner.push((Npc, Kind::Npc, Position { x: 0, y: 2, map: 1 }, Hp(1), Name("jill".to_string())));
  world.inner.push((Npc, Kind::Npc, Position { x: 0, y: 5, map: 1 }, Hp(1), Name("stephan".to_string())));
  world.inner.push((Npc, Kind::Npc, Position { x: 2, y: 0, map: 2 }, Hp(1), Name("jynx".to_string())));

  world.inner.push((Player, Kind::Player, Position { x: 2, y: 10, map: 1 }, Hp(1), Name("genusis".to_string()), Weapon::Sword));

  // Damage all npcs.
  /*let mut query = <(&mut Hp,)>::query().filter(component::<Npc>());

  for (hp,) in query.iter_mut(&mut world.inner) {
      hp.0 -= 0.2;
  }

  // Print the health of all entities.
  let mut query = <(&Kind, &Hp,)>::query();

  for (kind, hp,) in query.iter_mut(&mut world.inner) {
      println!("{:?} = {:?}", kind, hp);
  }*/

  let testscript = match RuneNPCObject::new("test.rs") {
    Some(n) => n,
    None => {
      println!("Error did not load script or something derp.");
      return;
    }
  };

  let name = world.inner.entry(npcid).unwrap().into_component::<Name>().unwrap();
  println!("Name: {}", name.0);

  let mut id = LegionEntityWrap::new(npcid);
  let execution = match testscript.vm.call(&["calculate"], (&mut world, &mut id)) {
    Ok(n) => n,
    Err(e) => {
      println!("Error: {}", e);
      return;
    }
  };
  let value = i64::from_value(execution).unwrap();
  println!("{}", value);

  let name = world.inner.entry(id.inner).unwrap().into_component::<Name>().unwrap();
  println!("Name: {}", name.0);
}
