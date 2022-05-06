use bevy::prelude::{Res, Query, With, Entity, Commands};
use bevy_rapier2d::plugin::RapierContext;

use crate::models::explosion::ExplosionCore;

pub fn explosion_expand(_commands: Commands, explosion_query: Query<Entity, With<ExplosionCore>>, _rapier_context: Res<RapierContext>) {
    for _explosion_entity in explosion_query.iter() {
        
    }
}