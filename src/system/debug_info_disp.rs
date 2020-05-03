use crate::{components::DebugInfomationDisplay, traits::DebugDisplayFormat};
use amethyst::{
    core::Transform,
    ecs::{Entities, Entity, Join, ReadStorage, System, WriteStorage},
    ui::{Anchor, FontHandle, LineMode, UiText, UiTransform},
};
use std::{collections::BTreeMap, marker::PhantomData};

pub struct DebugInfomationDisplaySystem<T> {
    marker: PhantomData<T>,
    system_font: FontHandle,
    displayed: BTreeMap<Entity, Entity>,
}

impl<T> DebugInfomationDisplaySystem<T> {
    pub fn new(system_font: FontHandle) -> Self {
        DebugInfomationDisplaySystem {
            marker: PhantomData,
            system_font,
            displayed: BTreeMap::new(),
        }
    }
}

impl<'s, T> System<'s> for DebugInfomationDisplaySystem<T>
where
    T: DebugDisplayFormat<'s>,
{
    type SystemData = (
        WriteStorage<'s, UiText>,
        WriteStorage<'s, UiTransform>,
        ReadStorage<'s, DebugInfomationDisplay<T>>,
        ReadStorage<'s, Transform>,
        Entities<'s>,
        T::DisplayData,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut texts, mut ui_transforms, tags, transforms, entities, display_data) = data;
        // タグの付いたものだけ
        for (_, e, transform) in (&tags, &*entities, &transforms).join() {
            // 対応するUIがなければ作る
            let ui_entity = *self.displayed.entry(e).or_insert(entities.create());
            match update_ui::<T>(
                ui_entity,
                transform,
                &mut texts,
                &mut ui_transforms,
                self.system_font.clone(),
                &display_data,
            ) {
                Ok(()) => {}
                Err(err) => log::error!("error: {:?}", err),
            }
        }
    }
}

fn update_ui<'s, T>(
    entity: Entity,
    transform: &Transform,
    texts: &mut WriteStorage<'s, UiText>,
    transforms: &mut WriteStorage<'s, UiTransform>,
    system_font: FontHandle,
    display_data: &T::DisplayData,
) -> amethyst::Result<()>
where
    T: DebugDisplayFormat<'s>,
{
    let text = texts.entry(entity)?.or_insert(UiText::new(
        system_font,
        "".to_string(),
        [0., 0., 0., 1.],
        16.,
    ));
    text.line_mode = LineMode::Wrap;
    text.align = Anchor::TopLeft;

    let ui_transform = transforms.entry(entity)?.or_insert(UiTransform::new(
        format!("debug_{}_{}", entity.id(), entity.gen().id()),
        Anchor::Middle,
        Anchor::TopLeft,
        0.,
        0.,
        0.,
        200.,
        600.,
    ));
    ui_transform.local_x = transform.translation().x;
    ui_transform.local_y = transform.translation().y;
    ui_transform.local_z = transform.translation().z;

    text.text = T::display(entity, display_data);

    Ok(())
}
