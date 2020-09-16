use legion::*;
use runestick::Any;

#[derive(Any)]
pub struct LegionWrap {
  pub inner: legion::World,
}

impl LegionWrap {
  pub fn new() -> Self {
    Self {
        inner: World::default(),
    }
  }
}

#[derive(Any)]
pub struct LegionEntityWrap {
  pub inner: legion::Entity,
}

impl LegionEntityWrap {
  pub fn new(entity: legion::Entity) -> Self {
    Self {
        inner: entity,
    }
  }
}

pub fn wrap_modules() -> Result<runestick::Module, runestick::ContextError> {
  let mut module = runestick::Module::new(&["wrappers"]);

  module.ty::<LegionWrap>()?;
  module.ty::<LegionEntityWrap>()?;
  Ok(module)
}