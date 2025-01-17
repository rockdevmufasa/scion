use legion::{*};

use scion::core::resources::time::Timers;
use scion::core::{
    components::{animations::Animations, maths::transform::Transform},
    resources::inputs::{inputs_controller::InputsController, types::KeyCode},
};

use crate::scene::MainCharacter;

#[system(for_each)]
pub fn controller(
    #[resource] inputs: &mut InputsController,
    #[resource] timers: &mut Timers,
    character: &MainCharacter,
    transform: &Transform,
    animations: &mut Animations,
) {
    let no_delta = transform.translation().x() as usize % 48 == 0
        && transform.translation().y() as usize % 48 == 0;

    if !timers.get_timer("SceneSwitch").unwrap().ended() {
        return;
    }
    if !animations.any_animation_running() {
        if character.right && inputs.key_pressed(&KeyCode::Right) {
            animations.loop_animation("MOVE_RIGHT");
        } else if character.left && inputs.key_pressed(&KeyCode::Left) {
            animations.loop_animation("MOVE_LEFT");
        } else if character.top && inputs.key_pressed(&KeyCode::Up) {
            animations.loop_animation("MOVE_TOP");
        } else if character.bottom && inputs.key_pressed(&KeyCode::Down) {
            animations.loop_animation("MOVE_BOTTOM");
        }
    } else if no_delta {
        if !character.right
            && inputs.key_pressed(&KeyCode::Right)
            && animations.animation_running("MOVE_RIGHT")
        {
            println!("{:?} {}", character, animations.animation_running("MOVE_RIGHT"));
            animations.stop_all_animation(true);
        }
        if !character.left
            && inputs.key_pressed(&KeyCode::Left)
            && animations.animation_running("MOVE_LEFT")
        {
            println!("{:?} {}", character, animations.animation_running("MOVE_LEFT"));
            animations.stop_all_animation(true);
        }
        if !character.top
            && inputs.key_pressed(&KeyCode::Up)
            && animations.animation_running("MOVE_TOP")
        {
            println!("{:?} {}", character, animations.animation_running("MOVE_TOP"));
            animations.stop_all_animation(true);
        }
        if !character.bottom
            && inputs.key_pressed(&KeyCode::Down)
            && animations.animation_running("MOVE_BOTTOM")
        {
            println!("{:?} {}", character, animations.animation_running("MOVE_BOTTOM"));
            animations.stop_all_animation(true);
        }
    }

    inputs.on_key_released(KeyCode::Right, || {
        animations.stop_animation("MOVE_RIGHT", false);
    });
    inputs.on_key_released(KeyCode::Left, || {
        animations.stop_animation("MOVE_LEFT", false);
    });
    inputs.on_key_released(KeyCode::Up, || {
        animations.stop_animation("MOVE_TOP", false);
    });
    inputs.on_key_released(KeyCode::Down, || {
        animations.stop_animation("MOVE_BOTTOM", false);
    });

    if inputs.all_pressed().is_empty() {
        animations.stop_all_animation(false);
    }
}
