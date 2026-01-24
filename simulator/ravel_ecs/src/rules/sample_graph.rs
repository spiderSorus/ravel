use crate::multiverse::{Multiverse, Rule};
use crate::ravel::Ravel;
use bevy::prelude::*;

pub struct SampleGraphRule;

impl Rule for SampleGraphRule {
    fn apply(&self, world: &mut World) {
        let mut aggregate = [0i128; 64];
        let mut query = world.query::<&Ravel>();
        for ravel in query.iter(world) {
            for i in 0..64 {
                aggregate[i] += ravel.chord[i] as i128;
            }
        }
        
        // This is a bit of a hack. We're assuming the multiverse is a resource.
        // A better approach might be to have the rule mutate the multiverse directly.
        if let Some(mut multiverse) = world.get_resource_mut::<Multiverse>() {
            multiverse.global_chord = aggregate;
        }
    }
}
