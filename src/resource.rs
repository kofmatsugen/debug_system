use amethyst::{
    assets::{AssetStorage, Loader},
    ecs::{Read, ReadExpect, World},
    ui::{get_default_font, FontAsset, FontHandle},
};

pub struct DebugFont {
    pub system_font: FontHandle,
}

impl DebugFont {
    pub(crate) fn insert_world(world: &mut World) {
        let system_font = world.exec(
            |(loader, storage): (ReadExpect<Loader>, Read<AssetStorage<FontAsset>>)| {
                get_default_font(&loader, &storage)
            },
        );

        world.insert(DebugFont { system_font });
    }
}
