/// Results which can occur while using diff
#[derive(Debug)]
pub enum DiffResult {
    Ok,
    DifferenceNotSpecified(String),
    DifferenceStderr(String),
    DifferenceStdout(String),
    Trouble(String),
    InnerProblem(String),
}
/// Possible causes of failing tests
#[derive(Debug)]
pub enum TestFail {
    Valgrind(String),
    Compilation(String),
    Diff(DiffResult),
    InnerProblem(String),
    ProgramExitCode(),
    ValgrindExitCode(),
    CompilationExitCode,
}

impl TestFail {
    /**
    Gets a problem description
    */
    pub fn get_problem(&self) -> &str {
        match self {
            TestFail::Valgrind(err) => err,
            TestFail::InnerProblem(err) => err,
            TestFail::ProgramExitCode() => "Probably caused by unexpected exit of tested program.",
            TestFail::ValgrindExitCode() => {
                "Probably caused by unexpected exit of valgrind program."
            }
            TestFail::Diff(diff_error) => match diff_error {
                DiffResult::DifferenceStderr(err) => err,
                DiffResult::DifferenceStdout(err) => err,
                DiffResult::InnerProblem(err) => err,
                DiffResult::Trouble(err) => err,
                _ => "UNDEFINED BEHAVIOUR OF GET_PROBLEM FUNCTION",
            },
            TestFail::Compilation(comp_error) => comp_error,
            TestFail::CompilationExitCode => {
                "Probably caused by unexpected exit of compilation process (gcc)."
            }
        }
    }
}
