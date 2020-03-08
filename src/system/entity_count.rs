use amethyst::{
    ecs::*,
    ui::{UiFinder, UiText},
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
    type SystemData = (UiFinder<'s>, WriteStorage<'s, UiText>, Entities<'s>);

    fn run(&mut self, data: Self::SystemData) {
        self.update_ui(data);
    }
}

impl EntityCountSystem {
    fn update_ui(&mut self, data: <EntityCountSystem as System>::SystemData) -> Option<()> {
        let (finder, mut texts, entities) = data;

        if self.entity_count_ui.is_none() {
            self.entity_count_ui = finder.find("debug_entities_count");
        }
        let ui = self.entity_count_ui?;

        let mut count = 0;
        for _ in (&*entities,).join() {
            count = count + 1;
        }

        let text_ui = texts.get_mut(ui)?;
        text_ui.text = format!("entity: {}", count);

        Some(())
    }
}
