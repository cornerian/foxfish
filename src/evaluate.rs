use std::io;

use peppi::model::frame::{
	Frame,
	PortData
};
use peppi::model::primitives::{
	Position
};

use crate::stage;

pub type StockAdvantage = f32;

pub fn checkmate(port: &PortData, stage: peppi::model::enums::stage::Stage) -> bool {
	// If any velocity plus position is past a blast zone, return true
	// println!("Port: {:?}", port);

    let data = port.leader.post;

    let stage_data = stage::Stages::get_data(stage).unwrap();

    let current_velocity = data.velocities.unwrap();

    let next_position = Position {
        x: data.position.x + current_velocity.knockback.x + current_velocity.autogenous.x,
        y: data.position.y + current_velocity.knockback.y + current_velocity.autogenous.y,
    };

    println!("Character: {:?}", data.character);
    println!("Current position: {:?}", data.position);
    println!("Velocities: {:?}", current_velocity);
    println!("Next position: {:?}", next_position);
    println!("Blasts: {:?}", stage_data.blasts);

    if 
        next_position.x < stage_data.blasts.left || 
        next_position.x > stage_data.blasts.right || 
        next_position.y < stage_data.blasts.bottom || 
        next_position.y > stage_data.blasts.top 
    {
        println!("Checkmate!");
        return true;
    }

	return false;
}

pub fn position<const N: usize>(frame: &Frame<N>) -> StockAdvantage {
	// println!("Frame: {:?}", frame);

	return 0.;
}