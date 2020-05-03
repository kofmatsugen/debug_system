use amethyst::ecs::{Entity, SystemData};

pub trait DebugDisplayFormat<'s>: 'static + Send + Sync + Default {
    type DisplayData: SystemData<'s>;

    fn display(entity: Entity, display_data: &Self::DisplayData) -> Option<String>;
}

impl<'s> DebugDisplayFormat<'s> for () {
    type DisplayData = ();

    fn display(_: Entity, _: &Self::DisplayData) -> Option<String> {
        Some("Void Test".into())
    }
}
