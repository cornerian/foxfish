use peppi::model::frame::Frame;
use peppi::model::enums::stage::Stage;

pub mod evaluate;
pub mod stage;

pub struct Evaluator<const N: usize> {
    pub frame: Frame<N>,
    pub stage: Stage,
}

impl Evaluator<1> {
    pub fn evaluate(&self) -> evaluate::StockAdvantage {
        return 0.;
    }
}

impl Evaluator<2> {
    pub fn evaluate(&self) -> evaluate::StockAdvantage {
        if evaluate::checkmate(&self.frame.ports[1], self.stage) {
            return 1.;
        } else if evaluate::checkmate(&self.frame.ports[0], self.stage) {
            return -1.;
        }

        return 0.;
    }
}

impl Evaluator<3> {
    pub fn evaluate(&self) -> evaluate::StockAdvantage {
        return 0.;
    }
}

impl Evaluator<4> {
    pub fn evaluate(&self) -> evaluate::StockAdvantage {
        return 0.;
    }
}