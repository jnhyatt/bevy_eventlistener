#![deny(missing_docs)]

//! Event listening, bubbling, and callbacks.
//!
//! An implementation of event listeners and callbacks, allowing you to define behavior with
//! components.
//!
//! - Define custom events that can target entities.
//! - Add event listener components that run callbacks when the specified event reaches that entity.
//! - Define callbacks as normal bevy systems.
//! - Events bubble up entity hierarchies, allowing you to attach behavior to trees of entities. For
//!   example, you could put a single event listener on the root entity of a scene, that runs a
//!   callback if any child entity is clicked on. This works because events that target the child of
//!   a scene will bubble up the hierarchy until an event listener is found.
//!
//! ## Example
//!
//! Taken from the `minimal` example, here we have a goblin wearing a few pieces of armor. An
//! `Attack` event can target any of these entities. If an `Attack` reaches a piece of armor, the
//! armor will try to absorb the attack. Any damage it is not able to absorb will bubble to the
//! goblin wearing the armor.
//!
//! ```
//! # use bevy::prelude::*;
//! use bevy_eventlistener::prelude::*;
//!
//! # #[derive(Clone, Event, EntityEvent)]
//! # struct Attack {
//! #     #[target]
//! #     target: Entity,
//! # }
//! # fn take_damage() {}
//! # fn block_attack() {}
//! # fn system(mut commands: Commands) {
//! commands
//!     .spawn((
//!         Name::new("Goblin"),
//!         On::<Attack>::run(take_damage),
//!     ))
//!     .with_children(|parent| {
//!         parent.spawn((
//!             Name::new("Helmet"),
//!             On::<Attack>::run(block_attack),
//!         ));
//!         parent.spawn((
//!             Name::new("Socks"),
//!             On::<Attack>::run(block_attack),
//!         ));
//!     });
//! # }
//! ```
//!
//! ## UI
//!
//! This library is intended to be upstreamed to bevy for use in making interactive UI. However, as
//! demonstrated above, event bubbling is applicable to any kind of event that needs to traverse an
//! entity hierarchy. This follows the basic principles of ECS patterns: it works on *any* entity
//! with the required components, not just UI.
//!
//! This library was initially extracted from the `0.13` version of bevy_mod_picking, as it became
//! obvious that this is a generically useful feature.
//!
//! ## Performance
//!
//! Using DOM data from the most complex websites I could find, the stress test example was built to
//! help benchmark the performance of this implementation with a representative dataset. Using a DOM
//! complexity:
//! - Depth: 64 (how many levels of children for an entity at the root)
//! - Total nodes: 12,800 (total number of entities spawned)
//! - Listener density: 20% (what percent of entities have event listeners?)
//! ![image](https://github.com/aevyrie/bevy_eventlistener/assets/2632925/72f75640-8b44-4ace-af67-9898c4c78321)
//!
//! The blue line can be read as "how long does it take all of these events to bubble up a hierarchy
//! and trigger callbacks at ~20% of the 64 nodes as it traverses depth?". A graph is built for
//! every event as an acceleration structure, which allows us to have linearly scaling performance.
//!
//! The runtime cost of each event decreases as the total number of events increase, this is because
//! graph construction is a fixed cost for each type of event. Adding more events simply amortizes
//! that cost across more events. At 50 events the runtime cost is only ~500ns/event, and about 25us
//! total. To reiterate, this is using an entity hierarchy similar to the most complex websites I
//! could find.

pub use bevy_eventlistener_core::*;
pub use bevy_eventlistener_derive::EntityEvent;

/// Common exports
pub mod prelude {
    pub use bevy_eventlistener_core::{
        callbacks::{Listener, ListenerInput, ListenerMut},
        event_listener::{EntityEvent, On},
        EventListenerPlugin,
    };
    pub use bevy_eventlistener_derive::EntityEvent;
}
