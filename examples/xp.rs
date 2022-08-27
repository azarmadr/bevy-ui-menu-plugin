use bevy::prelude::*;
#[cfg(feature = "debug")]
use bevy_inspector_egui::{InspectorPlugin, WorldInspectorPlugin};
use menu_plugin::*;

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
struct A(u8, u8);
impl MenuItem for A {
    type Attributes = ();
    fn ui(&self, cmd: &mut Commands, m: &Res<MenuMaterials>, _opts: Self::Attributes) -> Entity {
        let e0 = self.0.ui(
            cmd,
            m,
            Volume::<u8> {
                max: Some(27),
                vertical: false,
                prefix: "A ".to_string(),
                ..default()
            },
        );
        let e1 = self.1.ui(
            cmd,
            m,
            Volume::<u8> {
                max: Some(3),
                vertical: true,
                prefix: "B ".to_string(),
                ..default()
            },
        );
        cmd.spawn_bundle(m.button_border())
            .push_children(&[e0, e1])
            .id()
    }
    fn system(&mut self, e: Entity, m: &MenuMaterials, world: &mut World) {
        let mut ch = world.query::<&mut Children>();
        let chv: Vec<Entity> = ch.get(world, e).unwrap()[..].to_vec();
        self.0.system(chv[0], m, world);
        self.1.system(chv[1], m, world);
    }
}

fn main() {
    let mut app = App::new();
    app.insert_resource(WindowDescriptor {
        title: "Menu Plugin Example - Bevy".to_string(),
        width: 1080.,
        height: 720.,
        ..Default::default()
    })
    .insert_resource(A(8u8, 1))
    .add_plugins(DefaultPlugins)
    .init_resource::<MenuMaterials>()
    .add_plugin(MenuPlugin::<A>(std::marker::PhantomData))
    .add_startup_system(startup);
    // Debug hierarchy inspector
    #[cfg(feature = "debug")]
    app.add_plugin(WorldInspectorPlugin::new())
        .add_plugin(InspectorPlugin::<u8>::new())
        .add_plugin(InspectorPlugin::<A>::new_insert_manually())
        .add_plugin(InspectorPlugin::<MenuMaterials>::new());
    app.run();
}
fn startup(mut commands: Commands) {
    commands.spawn_bundle(Camera3dBundle::default());
}
