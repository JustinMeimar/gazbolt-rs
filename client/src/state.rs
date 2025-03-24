use crate::compiler::Compiler;
use std::rc::Rc;
use yew::Reducible;
use web_sys::console;

#[derive(PartialEq, Clone)]
pub struct AppState {
    pub code: String,
    pub compiler_option: Compiler,
    pub stdin: String,
    pub stdout: String,
    pub stderr: String,
}

pub enum AppAction {
    UpdateCode(String),
    UpdateCompiler(Compiler),
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
            AppAction::UpdateCompiler(compiler) => next_state.compiler_option = compiler,
            AppAction::UpdateStdin(stdin) => next_state.stdin = stdin,
            AppAction::UpdateStdout(stdout) => next_state.stdout = stdout,
            AppAction::UpdateStderr(stderr) => next_state.stderr = stderr,
        }
        
        next_state.into()
    }
}
