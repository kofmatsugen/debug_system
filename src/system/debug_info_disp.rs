use crate::{components::DebugInfomationDisplay, traits::DebugDisplayFormat};
use amethyst::{
    core::Transform,
    ecs::{Entities, Entity, Join, ReadStorage, System, WriteStorage},
    renderer::debug_drawing::DebugLinesComponent,
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
        WriteStorage<'s, DebugLinesComponent>,
        ReadStorage<'s, DebugInfomationDisplay<T>>,
        ReadStorage<'s, Transform>,
        Entities<'s>,
        T::DisplayData,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut texts,
            mut ui_transforms,
            mut debug_lines,
            tags,
            transforms,
            entities,
            display_data,
        ) = data;
        // タグの付いたものだけ
        for (_, e, transform) in (&tags, &*entities, &transforms).join() {
            // 対応するUIがなければ作る
            let mut ui_entity = self.displayed.get(&e).map(|e| *e);
            if ui_entity.is_none() {
                let entity = entities.create();
                self.displayed.insert(e, entity);
                ui_entity = Some(entity);
            }

            match update_ui::<T>(
                e,
                ui_entity.unwrap(),
                transform,
                &mut texts,
                &mut ui_transforms,
                self.system_font.clone(),
                &display_data,
            ) {
                Ok(()) => {}
                Err(err) => log::error!("error: {:?}", err),
            }

            match update_debug_lines::<T>(e, &mut debug_lines, &display_data) {
                Ok(()) => {}
                Err(err) => log::error!("error: {:?}", err),
            }
        }
    }
}

fn update_debug_lines<'s, T>(
    base_entity: Entity,
    debug_lines: &mut WriteStorage<'s, DebugLinesComponent>,
    display_data: &T::DisplayData,
) -> amethyst::Result<()>
where
    T: DebugDisplayFormat<'s>,
{
    let debug_lines = debug_lines
        .entry(base_entity)?
        .or_insert(DebugLinesComponent::with_capacity(32));
    debug_lines.clear();

    T::debug_lines(base_entity, debug_lines, display_data, 1023.);
    Ok(())
}

fn update_ui<'s, T>(
    base_entity: Entity,
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
        600.,
        600.,
    ));
    ui_transform.local_x = transform.translation().x;
    ui_transform.local_y = transform.translation().y;
    ui_transform.local_z = transform.translation().z;

    text.text = T::display(base_entity, display_data).unwrap_or("".to_string());

    Ok(())
}
