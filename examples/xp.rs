use bevy::prelude::*;
#[cfg(feature = "dev")]
use bevy_inspector_egui::{InspectorPlugin, WorldInspectorPlugin};
use menu_plugin::*;

#[cfg_attr(feature = "dev", derive(bevy_inspector_egui::Inspectable))]
#[derive(Resource)]
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
        cmd.spawn(m.button_border()).push_children(&[e0, e1]).id()
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
    app.insert_resource(Msaa { samples: 1 })
        .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Menu Plugin Example - Bevy".to_string(),
                width: 1080.,
                height: 720.,
                ..Default::default()
            },
            ..default()
        }))
        .insert_resource(A(8u8, 1))
        .init_resource::<MenuMaterials>()
        .add_plugin(MenuPlugin::<A>::default())
        .add_startup_system(startup);
    // dev hierarchy inspector

    #[cfg(feature = "dev")]
    app.add_plugin(WorldInspectorPlugin::new())
        .add_plugin(InspectorPlugin::<A>::new_insert_manually())
        .add_plugin(InspectorPlugin::<MenuMaterials>::new());

    app.run();
}
fn startup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
