use amethyst::{
    assets::*,
    ecs::*,
    ui::{get_default_font, Anchor, FontAsset, FontHandle, UiText, UiTransform},
};

use log::*;

pub struct EntityCountSystem {
    system_font: Option<FontHandle>,
    entity_count_ui: Option<Entity>,
}

impl EntityCountSystem {
    pub fn new() -> Self {
        EntityCountSystem {
            system_font: None,
            entity_count_ui: None,
        }
    }
}

impl<'s> System<'s> for EntityCountSystem {
    type SystemData = (
        ReadExpect<'s, Loader>,
        Read<'s, AssetStorage<FontAsset>>,
        WriteStorage<'s, UiTransform>,
        WriteStorage<'s, UiText>,
        Entities<'s>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (loader, storage, mut transforms, mut texts, entities) = data;

        let mut count = 0;
        for _ in (&*entities,).join() {
            count = count + 1;
        }

        if let Some(_) = &self.system_font {
        } else {
            let handle = get_default_font(&loader, &storage);
            info!("set default font: {:?}", handle);
            self.system_font = Some(handle);
        }

        let system_font = self.system_font.as_ref().unwrap().clone();
        let count_text = format!("entity: {}", count);

        if let Some(ui) = &self.entity_count_ui {
            let text = texts.get_mut(*ui).unwrap();
            text.text = count_text;
        } else {
            let entity = entities.create();
            let transform = UiTransform::new(
                "entity_count".to_string(),
                Anchor::TopLeft,
                Anchor::TopLeft,
                0.,
                0.,
                0.,
                200.,
                16.,
            );
            let text = UiText::new(system_font, count_text, [0., 0., 0., 1.], 16.);

            let _transform = transforms.insert(entity, transform);
            let _text = texts.insert(entity, text);
            self.entity_count_ui = Some(entity);
        }
    }
}
