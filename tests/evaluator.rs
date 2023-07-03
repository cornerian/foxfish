use std::{fs, io};

use peppi;
use peppi::model::game::Frames;

use foxfish::evaluate::evaluation;
use foxfish::evaluate::FrameTrait;

#[test]
fn it_evaluates() {
    let mut buf = io::BufReader::new(
        fs::File::open("game.slp").unwrap());
    let game = peppi::game(&mut buf, None, None).unwrap();

	let frame_idx = 0; // replace 0 with the index of the frame you want

	match game.frames {
		Frames::P1(frames) => {
			let frame = frames.get(frame_idx).unwrap();
			let score = frame.evaluate(evaluation);
			// Do something with score
		},
		Frames::P2(frames) => {
			let frame = frames.get(frame_idx).unwrap();
			let score = frame.evaluate(evaluation);
			// Do something with score
		},
		Frames::P3(frames) => {
			let frame = frames.get(frame_idx).unwrap();
			let score = frame.evaluate(evaluation);
			// Do something with score
		},
		Frames::P4(frames) => {
			let frame = frames.get(frame_idx).unwrap();
			let score = frame.evaluate(evaluation);
			// Do something with score
		},
	}
}