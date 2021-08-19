use super::*;
use crate::harmonics;
use crate::util::test::parse_csv;

use alloc::vec::Vec;

pub fn test_process_step_expected() -> impl Iterator<Item = impl Iterator<Item = f64> + ExactSizeIterator> {
    parse_csv(include_bytes!("test/process_step/candidates.csv"))
}

#[test]
fn test_process_step() {
    let expected_steps = test_process_step_expected();

    let sample_rate = 6000.0;

    let mut harmonics = harmonics::test::test_process_step_expected();
    let mut candidate_generator = CandidateGenerator::new(16384, 12, 2, sample_rate, 50.0..=450.0);

    for (step_index, expected_step) in expected_steps.enumerate() {
        let candidates = candidate_generator.process_step(harmonics.next().unwrap(), sample_rate);
        let candidates = candidates.collect::<Vec<_>>();
        assert_iter_approx_eq!(candidates, expected_step, 1e-7%, "step {}", step_index);
    }
}
