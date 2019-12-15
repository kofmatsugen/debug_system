use crate::resource::DebugFont;
use amethyst::{
    ecs::*,
    ui::{Anchor, UiText, UiTransform},
};

pub struct EntityCountSystem {
    entity_count_ui: Option<Entity>,
}

impl EntityCountSystem {
    pub fn new() -> Self {
        EntityCountSystem {
            entity_count_ui: None,
        }
    }
}

impl<'s> System<'s> for EntityCountSystem {
    type SystemData = (
        ReadExpect<'s, DebugFont>,
        WriteStorage<'s, UiTransform>,
        WriteStorage<'s, UiText>,
        Entities<'s>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (debug_font, mut transforms, mut texts, entities) = data;

        let mut count = 0;
        for _ in (&*entities,).join() {
            count = count + 1;
        }

        let system_font = debug_font.system_font.clone();
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
