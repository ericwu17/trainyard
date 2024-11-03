use bevy::prelude::*;

#[derive(Component)]
pub struct SrinkToNoneAnimationComponent(pub f32);

pub struct TileAnimationPlugin;

impl Plugin for TileAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, shrink_to_zero_animation_system);
    }
}

fn shrink_to_zero_animation_system(
    mut commands: Commands,
    mut query: Query<(
        &Parent,
        Entity,
        &mut Transform,
        &mut SrinkToNoneAnimationComponent,
    )>,
    time: Res<Time>,
) {
    for (parent, entity, mut xf, mut shrink) in query.iter_mut() {
        let scale = f32::powf(0.00005, time.delta_seconds());
        *xf = xf.mul_transform(Transform::from_scale(Vec3::new(scale, scale, 1.0)));

        shrink.0 *= scale;

        if shrink.0 < 0.001 {
            // despawn the entity, since we assume it is now too small to see
            commands.entity(parent.get()).remove_children(&[entity]);
            commands.entity(entity).despawn_recursive();
        }
    }
}
