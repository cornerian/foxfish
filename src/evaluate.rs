use std::io;

use peppi::model::frame::Frame;

type StockAdvantage = f32;

pub trait FrameTrait {
    fn evaluate<F: Fn(&Self) -> StockAdvantage>(&self, evaluator: F) -> StockAdvantage;
}

impl FrameTrait for Frame<1> {
    fn evaluate<F: Fn(&Self) -> StockAdvantage>(&self, evaluator: F) -> StockAdvantage {
		return 0.;
    }
}

impl FrameTrait for Frame<2> {
    fn evaluate<F: Fn(&Self) -> StockAdvantage>(&self, evaluator: F) -> StockAdvantage {
        evaluator(&self)
    }
}

impl FrameTrait for Frame<3> {
    fn evaluate<F: Fn(&Self) -> StockAdvantage>(&self, evaluator: F) -> StockAdvantage {
        evaluator(&self)
    }
}

impl FrameTrait for Frame<4> {
    fn evaluate<F: Fn(&Self) -> StockAdvantage>(&self, evaluator: F) -> StockAdvantage {
        evaluator(&self)
    }
}

pub fn evaluation<const N: usize>(frame: &Frame<N>) -> StockAdvantage {
	return 0.;
}