use peppi::model::enums::stage::Stage;
use peppi::model::primitives::Position;

#[derive(Debug)]
pub struct PlatformData {
	pub position: Position,
	pub length: f32,
}

#[derive(Debug)]
pub struct BlastData {
	pub top: f32,
	pub bottom: f32,
	pub left: f32,
	pub right: f32,
}

#[derive(Debug)]
pub struct StageData {
	pub length: f32,
	pub blasts: BlastData, // top, bottom, left, right
	pub platforms: &'static [PlatformData],
}

#[non_exhaustive]
pub struct Stages;

// https://smashboards.com/threads/stage-blast-zones-via-debug-mode.319898/
// https://smashboards.com/threads/official-ask-anyone-frame-things-thread.313889/page-20#post-18643652
impl Stages {
	pub fn get_data(stage: Stage) -> Option<StageData> {
		match stage {
			Stage::FOUNTAIN_OF_DREAMS => Some(StageData {
				length: 128.85448,
				blasts: BlastData {
					top: 202.50, 
					bottom: -146.25,
					left: -198.75, 
					right: 198.75,
				},
				platforms: &[],
			}),
			Stage::POKEMON_STADIUM => Some(StageData {
				length: 175.5,
				blasts: BlastData {
					top: 180.00, 
					bottom: -111.00,
					left: -230.00, 
					right: 230.00,
				},
				platforms: &[],
			}),
			Stage::YOSHIS_STORY => Some(StageData {
				length: 112.6327,
				blasts: BlastData {
					top: 168.00, 
					bottom: -91.00,
					left: -175.70, 
					right: 173.60,
				},
				platforms: &[], // No.
			}),
			Stage::DREAM_LAND_N64 => Some(StageData {
				length: 128.85448,
				blasts: BlastData {
					top: 250.00, 
					bottom: -123.00,
					left: -255.00, 
					right: 255.00,
				},
				platforms: &[],
			}),
			Stage::BATTLEFIELD => Some(StageData {
				length: 136.8,
				blasts: BlastData {
					top: 200.00, 
					bottom: -108.80,
					left: -224.00, 
					right: 224.00,
				},
				platforms: &[],
			}),
			Stage::FINAL_DESTINATION => Some(StageData {
				length: 171.1314,
				blasts: BlastData {
					top: 188.00, 
					bottom: -140.00,
					left: -246.00, 
					right: 246.00,
				},
				platforms: &[],
			}),
			_ => None,
		}	
	}
}