use bevy::prelude::*;
use rand::random;

#[derive(Component)]
pub struct SrinkToNoneAnimationComponent(pub f32);

#[derive(Component)]
pub struct FloatingFadingAnimationComponent {
    pub ttl: f32,
    pub v_x: f32,
    pub v_y: f32,
}

impl FloatingFadingAnimationComponent {
    pub fn new() -> Self {
        Self {
            ttl: 1.0,
            v_x: (random::<f32>() - 0.5) * 40.0,
            v_y: (random::<f32>() - 0.5) * 40.0,
        }
    }
}

pub struct TileAnimationPlugin;

impl Plugin for TileAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (shrink_to_zero_animation_system, float_fade_animation_system),
        );
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

fn float_fade_animation_system(
    mut commands: Commands,
    mut query: Query<(
        &Parent,
        Entity,
        &mut Transform,
        &mut Sprite,
        &mut FloatingFadingAnimationComponent,
    )>,
    time: Res<Time>,
) {
    for (parent, entity, mut xf, mut sprite, mut fade) in query.iter_mut() {
        let dx = fade.v_x * time.delta_seconds();
        let dy = fade.v_y * time.delta_seconds();

        *xf = *xf * Transform::from_xyz(dx, dy, 0.0);

        fade.ttl -= time.delta_seconds() * 0.7;

        let mut color = sprite.color.to_srgba();
        color.alpha = fade.ttl;
        sprite.color = color.into();

        if fade.ttl <= 0.0 {
            // despawn the entity, since we assume it is now too small to see
            commands.entity(parent.get()).remove_children(&[entity]);
            commands.entity(entity).despawn_recursive();
        }
    }
}
