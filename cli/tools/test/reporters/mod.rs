// Copyright 2018-2023 the Deno authors. All rights reserved. MIT license.

use super::*;

mod common;
mod compound;
mod dot;
mod junit;
mod pretty;

pub use compound::CompoundTestReporter;
pub use dot::DotTestReporter;
pub use junit::JunitTestReporter;
pub use pretty::PrettyTestReporter;

pub trait TestReporter {
  fn report_register(&mut self, description: &TestDescription);
  fn report_plan(&mut self, plan: &TestPlan);
  fn report_wait(&mut self, description: &TestDescription);
  fn report_output(&mut self, output: &[u8]);
  fn report_result(
    &mut self,
    description: &TestDescription,
    result: &TestResult,
    elapsed: u64,
  );
  fn report_uncaught_error(&mut self, origin: &str, error: Box<JsError>);
  fn report_step_register(&mut self, description: &TestStepDescription);
  fn report_step_wait(&mut self, description: &TestStepDescription);
  fn report_step_result(
    &mut self,
    desc: &TestStepDescription,
    result: &TestStepResult,
    elapsed: u64,
    tests: &IndexMap<usize, TestDescription>,
    test_steps: &IndexMap<usize, TestStepDescription>,
  );
  fn report_summary(
    &mut self,
    elapsed: &Duration,
    tests: &IndexMap<usize, TestDescription>,
    test_steps: &IndexMap<usize, TestStepDescription>,
  );
  fn report_sigint(
    &mut self,
    tests_pending: &HashSet<usize>,
    tests: &IndexMap<usize, TestDescription>,
    test_steps: &IndexMap<usize, TestStepDescription>,
  );
  fn flush_report(
    &mut self,
    elapsed: &Duration,
    tests: &IndexMap<usize, TestDescription>,
    test_steps: &IndexMap<usize, TestStepDescription>,
  ) -> anyhow::Result<()>;
}
