//===----------------------------------------------------------------------===//
// ~ file: state.rs
// ~ author: Justin Meimar
// ~ date: march 25th 2025
// ~ desc: Frontend state structure and interface for manipulating in a
//         functionally pure way, like Javascript frameworks are disposed to.
//===----------------------------------------------------------------------===//

use std::rc::Rc;
use yew::Reducible;

#[derive(PartialEq, Clone)]
pub struct AppState {
    pub code: String,
    pub selected_compiler: String,
    pub stdin: String,
    pub stdout: String,
    pub stderr: String,
}

pub enum AppAction {
    UpdateCode(String),
    UpdateCompiler(String),
    UpdateStdin(String),
    UpdateStdout(String),
    UpdateStderr(String),
}

impl Reducible for AppState {
    type Action = AppAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut next_state = (*self).clone();

        match action {
            AppAction::UpdateCode(code) => next_state.code = code,
            AppAction::UpdateCompiler(compiler) => next_state.selected_compiler = compiler,
            AppAction::UpdateStdin(stdin) => next_state.stdin = stdin,
            AppAction::UpdateStdout(stdout) => next_state.stdout = stdout,
            AppAction::UpdateStderr(stderr) => next_state.stderr = stderr,
        }
        next_state.into()
    }
}
