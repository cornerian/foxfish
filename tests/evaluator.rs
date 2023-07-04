use std::{fs, io};

use peppi;
use peppi::model::game::Frames;

use foxfish::Evaluator;

#[test]
fn it_evaluates() {
    let mut buf = io::BufReader::new(
        fs::File::open("game.slp").unwrap());
    let game = peppi::game(&mut buf, None, None).unwrap();

	let frame_idx = 592; // index is super weird here

	match game.frames {
		Frames::P1(frames) => {
			let frame = frames.get(frame_idx).unwrap();
			let evaluator = Evaluator::<1> {
				frame: frame.clone(),
				stage: game.start.stage,
			};
			let score = evaluator.evaluate();
			// Do something with score
		},
		Frames::P2(frames) => {
			let frame = frames.get(frame_idx).unwrap();
			let evaluator = Evaluator::<2> {
				frame: frame.clone(),
				stage: game.start.stage,
			};
			println!("Index: {}", frame.index);
			let score = evaluator.evaluate();
			// Do something with score
		},
		Frames::P3(frames) => {
			let frame = frames.get(frame_idx).unwrap();
			let evaluator = Evaluator::<3> {
				frame: frame.clone(),
				stage: game.start.stage,
			};
			let score = evaluator.evaluate();
			// Do something with score
		},
		Frames::P4(frames) => {
			let frame = frames.get(frame_idx).unwrap();
			let evaluator = Evaluator::<4> {
				frame: frame.clone(),
				stage: game.start.stage,
			};
			let score = evaluator.evaluate();
			// Do something with score
		},
	}
}


use peppi::model::frame;
use peppi::serde::de::{Handlers, FrameEvent, PortId};
use peppi::model::enums::action_state::{
	State,
	Common
};

struct FramePrinter {}

impl Handlers for FramePrinter {
    fn frame_post(&mut self, post: FrameEvent<PortId, frame::Post>) -> io::Result<()> {
		match post.event.state {
			State::Common(common) => match common {
				Common::DEAD_DOWN => {
					// handle DEAD_DOWN state
					println!("DEAD_DOWN at frame {}", post.id.index);
					()
				},
				_ => {
					()
				}
			},
			_ => {
				()
			}
		}
		
        Ok(())
    }
}

#[test]
fn find_deaths() {
    let f = fs::File::open("game.slp").unwrap();
    let mut r = io::BufReader::new(f);
    let mut handlers = FramePrinter {};

    peppi::parse(&mut r, &mut handlers, None).unwrap();
}