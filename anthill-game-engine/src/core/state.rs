use anthill_di::Injection;



#[derive(Clone, Debug)]
pub enum State {
    CreateProject
}

pub struct CoreState {
    state: State,
}

impl Injection for CoreState {
    fn build_injection(injector: &mut anthill_di::Injector) -> Result<Self, anthill_di::DiError> {
        Ok(Self{state: State::CreateProject})
    }
}

impl CoreState {
    pub fn get_state(&self) -> State {
        self.state.clone()
    }

    pub fn set_state(&mut self, state: State) {
        self.state = state;
    }
}