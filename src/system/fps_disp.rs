use amethyst::{
    assets::*,
    ecs::*,
    ui::{get_default_font, Anchor, FontAsset, FontHandle, UiText, UiTransform},
    utils::fps_counter::FpsCounter,
};
use log::*;

pub struct FpsDispSystem {
    system_font: Option<FontHandle>,
    fps_disp_ui: Option<Entity>,
}

impl FpsDispSystem {
    pub fn new() -> Self {
        FpsDispSystem {
            system_font: None,
            fps_disp_ui: None,
        }
    }
}

impl<'s> System<'s> for FpsDispSystem {
    type SystemData = (
        ReadExpect<'s, Loader>,
        Read<'s, AssetStorage<FontAsset>>,
        WriteStorage<'s, UiTransform>,
        WriteStorage<'s, UiText>,
        Read<'s, FpsCounter>,
        Entities<'s>,
    );

    fn setup(&mut self, res: &mut Resources) {
        log::debug!("setup");
        Self::SystemData::setup(res);
    }

    fn run(&mut self, data: Self::SystemData) {
        let (loader, storage, mut transforms, mut texts, fps, entities) = data;

        match &self.system_font {
            Some(_) => {}
            None => {
                let handle = get_default_font(&loader, &storage);
                log::info!("set default font: {:?}", handle);
                self.system_font = Some(handle);
            }
        }

        let system_font = self.system_font.as_ref().unwrap().clone();
        let count_text = format!("fps: {:.2}", fps.sampled_fps());

        if let Some(ui) = &self.fps_disp_ui {
            let text = texts.get_mut(*ui).unwrap();
            text.text = count_text;
        } else {
            let entity = entities.create();
            let transform = UiTransform::new(
                "fps_disp".to_string(),
                Anchor::TopLeft,
                Anchor::TopLeft,
                0.,
                -32.,
                0.,
                200.,
                16.,
            );
            let text = UiText::new(system_font, count_text, [0., 0., 0., 1.], 16.);

            let _transform = transforms.insert(entity, transform);
            let _text = texts.insert(entity, text);
            self.fps_disp_ui = Some(entity);
        }
    }
}
