use runestick::Any;
use crate::wrappers::{LegionWrap, LegionEntityWrap};

pub fn npc_module() -> Result<runestick::Module, runestick::ContextError> {
  let mut module = runestick::Module::new(&["npc"]);

  module.ty::<Npc>()?;

  module.function(&["Npc", "new"], Npc::new)?;
  module.inst_fn("getx", Npc::getx)?;
  module.inst_fn("gety", Npc::gety)?;
  module.inst_fn("getmap", Npc::getmap)?;
  module.inst_fn("gethp", Npc::gethp)?;
  module.inst_fn("getname", Npc::getname)?;
  module.inst_fn("setname", Npc::setname)?;
  Ok(module)
}

/// A tag, telling us that the entity is an npc.
#[derive(Clone, Copy, Debug, PartialEq, Any, Default)]
pub struct Npc;

/// A tag, telling us that the entity is a player.
#[derive(Clone, Copy, Debug, PartialEq, Any, Default)]
pub struct Player;

#[derive(Clone, Debug, PartialEq, Any, Default)]
pub struct Name(pub String);

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub map: i64,
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Hp(pub i32);

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Weapon {
    Sword,
    Unarmed,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Kind {
    Npc,
    Player,
}

impl Npc {
  pub fn new() -> Npc {
    Npc {}
  }

  pub fn getx(&self, world: &mut LegionWrap, id: &LegionEntityWrap) -> i32 {
    if let Some(entry) = world.inner.entry(id.inner) {
      match entry.into_component::<Position>() {
        Ok(pos) => return pos.x,
        Err(_) => return 0,
      };
    }

    0
  }

  pub fn gety(&self, world: &mut LegionWrap, id: &LegionEntityWrap) -> i32 {
    if let Some(entry) = world.inner.entry(id.inner) {
      match entry.into_component::<Position>() {
        Ok(pos) => return pos.y,
        Err(_) => return 0,
      };
    }

    0
  }

  pub fn getmap(world: &mut LegionWrap, id: &LegionEntityWrap) -> i64 {
    if let Some(entry) = world.inner.entry(id.inner) {
      match entry.into_component::<Position>() {
        Ok(pos) => return pos.map,
        Err(_) => return 0,
      };
    }

    0
  }

  pub fn gethp(world: &mut LegionWrap, id: &LegionEntityWrap) -> i32 {
    if let Some(entry) = world.inner.entry(id.inner) {
      match entry.into_component::<Hp>() {
        Ok(hp) => return hp.0,
        Err(_) => return 0,
      };
    }

    0
  }

  pub fn getname(&self, world: &mut LegionWrap, id: &LegionEntityWrap) -> String {
    if let Some(entry) = world.inner.entry(id.inner) {
      match entry.into_component_mut::<Name>() {
        Ok(npcname) => return npcname.0.clone(),
        Err(_) => return String::from("Empty"),
      };
    }

    String::from("Empty")
  }

  pub fn setname(&self, world: &mut LegionWrap, id: &LegionEntityWrap, name: String) {
    if let Some(entry) = world.inner.entry(id.inner) {
      match entry.into_component_mut::<Name>() {
        Ok(npcname) => npcname.0 = name,
        Err(_) => return,
      };
    }
  }
}
