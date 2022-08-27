mod action;
mod materials;
pub use {action::*, materials::MenuMaterials};
use {
    autodefault::autodefault,
    bevy::prelude::*,
    duplicate::duplicate_item,
    std::cmp::{max, min},
};
pub trait MenuItem {
    type Attributes: Default + Clone;

    fn ui(
        &self,
        parent: &mut Commands,
        materials: &Res<MenuMaterials>,
        opts: Self::Attributes,
    ) -> Entity;

    #[allow(unused_variables)]
    fn system(&mut self, entity: Entity, materials: &MenuMaterials, world: &mut World);
}
#[derive(Debug, Clone, Component)]
pub struct Volume<T> {
    pub min: Option<T>,
    pub max: Option<T>,
    pub speed: f32,
    pub vertical: bool,
    pub prefix: String,
    pub suffix: String,
}
impl<T> Default for Volume<T> {
    fn default() -> Self {
        Volume {
            min: None,
            max: None,
            speed: 1.0,
            vertical: false,
            prefix: "".to_string(),
            suffix: "".to_string(),
        }
    }
}
#[duplicate_item(num_t; [u8];)]
impl MenuItem for num_t {
    type Attributes = Volume<num_t>;
    fn ui(&self, cmd: &mut Commands, m: &Res<MenuMaterials>, opts: Volume<num_t>) -> Entity {
        cmd.spawn_bundle(if opts.vertical {
            m.menu_td()
        } else {
            m.menu_lr()
        })
        .with_children(|p| {
            p.spawn_bundle(m.button_border()).with_children(|p| {
                p.spawn_bundle(m.button()).with_children(|p| {
                    p.spawn_bundle(m.button_text("+"));
                });
            });
            p.spawn_bundle(m.button_text(format!("{}", self)));
            p.spawn_bundle(m.button_border()).with_children(|p| {
                p.spawn_bundle(m.button()).with_children(|p| {
                    p.spawn_bundle(m.button_text("-"));
                });
            });
        })
        .insert(opts)
        .id()
    }
    fn system(&mut self, e: Entity, m: &MenuMaterials, world: &mut World) {
        let o = world.entity(e).get::<Volume<num_t>>().unwrap().clone();
        let max_v = o.max.unwrap_or(num_t::MAX);
        let min_v = o.min.unwrap_or(num_t::MIN);

        let mut ch = world.query::<&mut Children>();
        let mut query = world.query::<(&Interaction, ChangeTrackers<Interaction>, &mut UiColor)>();
        let mut chv: Vec<Entity> = ch.get(world, e).unwrap()[..].to_vec();
        let text = chv.remove(1);
        chv[0] = ch.get(world, chv[0]).unwrap()[0];
        chv[1] = ch.get(world, chv[1]).unwrap()[0];
        let chv: [Entity; 2] = chv.try_into().unwrap();
        query
            .get_many_mut(world, chv)
            .unwrap()
            .iter_mut()
            .enumerate()
            .for_each(|(id, (ref interaction, tracker, ref mut color))| {
                if **interaction == Interaction::Clicked && tracker.is_changed() {
                    match id {
                        0 => *self = min(max_v, self.saturating_add(o.speed as num_t)),
                        1 => *self = max(min_v, self.saturating_sub(o.speed as num_t)),
                        _ => unreachable!(),
                    }
                }
                color.0 = if id == 0 && *self == max_v || id == 1 && *self == min_v {
                    m.button
                } else {
                    match interaction {
                        Interaction::Clicked => m.pressed,
                        Interaction::Hovered => m.hovered,
                        Interaction::None => m.button,
                    }
                };
            });
        if let Some(mut text) = world.entity_mut(text).get_mut::<Text>() {
            text.sections[0].value = format!("{}{}{}", o.prefix, self, o.suffix).to_string();
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum MenuState {
    InGame,
    Menu,
    Disabled,
}
use MenuState::*;
pub struct MenuPlugin<T: MenuItem>(pub std::marker::PhantomData<T>);
impl<T: MenuItem + Sync + Send + 'static> Plugin for MenuPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_state(Menu)
            .add_system_set(SystemSet::on_enter(InGame).with_system(setup_ui))
            .add_system_set(SystemSet::on_resume(InGame).with_system(setup_ui))
            .add_system_set(SystemSet::on_exit(InGame).with_system(despawn::<UI>))
            .add_system_set(
                SystemSet::on_enter(Menu)
                    .with_system(setup_menu::<T>)
                    .with_system(despawn::<UI>),
            )
            .add_system_set(
                SystemSet::on_update(Menu).with_system(menu_system::<T>.exclusive_system()),
            )
            .add_system_set(SystemSet::on_exit(Menu).with_system(despawn::<MenuUI>));
    }
}
pub struct UI;
#[derive(Component)]
pub struct MenuItemI<T>(std::marker::PhantomData<T>);
#[derive(Component)]
pub struct MenuUI;
fn setup_ui() {}
#[autodefault]
fn setup_menu<T>(mut cmd: Commands, materials: Res<MenuMaterials>, res: Res<T>)
where
    T: MenuItem + Sync + Send + 'static,
{
    let menu = cmd.spawn_bundle(materials.menu_td()).id();
    cmd.spawn_bundle(materials.root())
        .insert(MenuUI)
        .insert(Name::new("MenuUI"))
        .with_children(|parent| {
            parent
                .spawn_bundle(materials.border())
                .push_children(&[menu]);
        });
    let e = res.as_ref().ui(&mut cmd, &materials, default());
    cmd.entity(menu).push_children(&[e]);
    cmd.entity(e)
        .insert(MenuItemI::<T>(std::marker::PhantomData));
}
fn despawn<T>() {}
fn menu_system<T>(world: &mut World)
where
    T: MenuItem + Sync + Send + 'static,
{
    world.resource_scope(|world, mut mi: Mut<T>| {
        let mut e = world.query_filtered::<Entity, With<MenuItemI<T>>>();
        let e = e.iter(world).next().unwrap();
        world.resource_scope(|world, m: Mut<MenuMaterials>| {
            mi.system(e, &m, world);
        });
    });
}
