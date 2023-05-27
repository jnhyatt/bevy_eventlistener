use std::time::Duration;

use bevy::{log::LogPlugin, prelude::*, time::common_conditions::on_timer};
use rand::{seq::IteratorRandom, thread_rng, Rng};

use bevy_eventlistener::{
    callbacks::Listened,
    on_event::{EntityEvent, On},
    EventListenerPlugin,
};
use bevy_eventlistener_derive::EntityEvent;

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugin(LogPlugin::default())
        .add_plugin(EventListenerPlugin::<Attack>::default())
        .add_event::<Attack>()
        .add_startup_system(setup)
        .add_system(attack_armor.run_if(on_timer(Duration::from_millis(200))))
        .run();
}

/// An event used with event listeners must implement `EntityEvent` and `Clone`.
#[derive(Clone, EntityEvent)]
struct Attack {
    #[target] // Marks the field of the event that specifies the target entity
    target: Entity,
    damage: u16,
}

/// An entity that can take damage
#[derive(Component, Deref, DerefMut)]
struct HitPoints(u16);

/// For damage to reach the wearer, it must exceed the armor.
#[derive(Component, Deref)]
struct Armor(u16);

/// Set up the world
fn setup(mut commands: Commands) {
    commands
        .spawn((
            Name::new("Goblin"),
            HitPoints(50),
            On::<Attack>::run_callback(take_damage),
        ))
        .with_children(|parent| {
            parent.spawn((
                Name::new("Helmet"),
                Armor(5),
                On::<Attack>::run_callback(block_attack),
            ));
            parent.spawn((
                Name::new("Skirt"),
                Armor(10),
                On::<Attack>::run_callback(block_attack),
            ));
            parent.spawn((
                Name::new("Chainmail"),
                Armor(15),
                On::<Attack>::run_callback(block_attack),
            ));
        });

    commands
        .spawn((
            Name::new("Bandit"),
            HitPoints(50),
            On::<Attack>::run_callback(take_damage),
        ))
        .with_children(|parent| {
            parent.spawn((
                Name::new("Gloves"),
                Armor(5),
                On::<Attack>::run_callback(block_attack),
            ));
            parent.spawn((
                Name::new("Pants"),
                Armor(10),
                On::<Attack>::run_callback(block_attack),
            ));
            parent.spawn((
                Name::new("Jacket"),
                Armor(15),
                On::<Attack>::run_callback(block_attack),
            ));
        });
}

/// A normal bevy system that attacks a piece of armor on a timer.
fn attack_armor(entities: Query<Entity, With<Armor>>, mut attacks: EventWriter<Attack>) {
    let mut rng = rand::thread_rng();
    if let Some(entity) = entities.iter().choose(&mut rng) {
        attacks.send(Attack {
            target: entity,
            damage: thread_rng().gen_range(1..20),
        });
    }
}

/// A callback placed on [`Armor`], checking if it absorbed all the [`Attack`] damage.
fn block_attack(mut attack: ResMut<Listened<Attack>>, armor: Query<(&Armor, &Name)>) {
    let (armor, armor_name) = armor.get(attack.target).unwrap();
    let damage = attack.damage.saturating_sub(**armor);
    if damage > 0 {
        info!("HIT: {} damage passed through {}", damage, armor_name);
        attack.damage = damage;
    } else {
        info!("BLOCK: {} blocked an attack.", armor_name);
        attack.stop_propagation(); // Armor stopped the attack, the event stops here.
    }
}

/// A callback on the armor wearer, triggered when a piece of armor is not able to block an attack.
fn take_damage(
    attack: Res<Listened<Attack>>,
    mut hp: Query<(&mut HitPoints, &Name)>,
    mut commands: Commands,
) {
    let (mut hp, name) = hp.get_mut(attack.listener()).unwrap();
    **hp = hp.saturating_sub(attack.damage);

    if **hp > 0 {
        info!("Ouch! {} has {:.1} HP.", name, hp.0);
    } else {
        warn!("{} has died a gruesome death.", name);
        commands.entity(attack.listener()).despawn_recursive()
    }
}