use std::io;

use bonc::stages::Stages;

use peppi::model::frame::{
	Frame,
	PortData
};
use peppi::model::primitives::{
	Position
};

pub type StockAdvantage = f32;

pub fn checkmate(port: &PortData, stage: peppi::model::enums::stage::Stage) -> bool {
	// If any velocity plus position is past a blast zone, return true

    let data = port.leader.post;

    let stage_data = Stages::get_data(stage).unwrap();

    let current_velocity = data.velocities.unwrap();

    let next_position = Position {
        x: data.position.x + current_velocity.knockback.x + current_velocity.autogenous.x,
        y: data.position.y + current_velocity.knockback.y + current_velocity.autogenous.y,
    };

    if 
        next_position.x < stage_data.blasts.left || 
        next_position.x > stage_data.blasts.right || 
        next_position.y < stage_data.blasts.bottom || 
        next_position.y > stage_data.blasts.top 
    {
        return true;
    }

	return false;
}

pub fn percentage_advantage(frame: &Frame<2>) -> StockAdvantage {
	// Players at a higher percentage are at a disadvantage

	return 0.;
}

pub fn positional_advantage(frame: &Frame<2>, stage: peppi::model::enums::stage::Stage) -> StockAdvantage {
    // Player further from center of stage is at a disadvantage

    // Player further below the stage is at a disadvantage

	return 0.;
}

pub fn frame_advantage(frame: &Frame<2>) -> StockAdvantage {
	// Players stuck in a commited action are at a disadvantage

	return 0.;
}

pub fn intangibility_advantage(frame: &Frame<2>) -> StockAdvantage {
	// Players close to an intangible player are at a disadvantage

	return 0.;
}