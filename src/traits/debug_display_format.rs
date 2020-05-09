use amethyst::{
    ecs::{Entity, SystemData},
    renderer::debug_drawing::DebugLinesComponent,
};

pub trait DebugDisplayFormat<'s>: 'static + Send + Sync + Default {
    type DisplayData: SystemData<'s>;

    fn display(entity: Entity, display_data: &Self::DisplayData) -> Option<String>;

    fn debug_lines(
        entity: Entity,
        debug_lines: &mut DebugLinesComponent,
        display_data: &Self::DisplayData,
        position_z: f32,
    ) -> Option<()>;
}

impl<'s> DebugDisplayFormat<'s> for () {
    type DisplayData = ();

    fn display(_: Entity, _: &Self::DisplayData) -> Option<String> {
        None
    }

    fn debug_lines(
        _: Entity,
        _: &mut DebugLinesComponent,
        _: &Self::DisplayData,
        _: f32,
    ) -> Option<()> {
        None
    }
}
