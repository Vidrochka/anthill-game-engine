use anthill_di::Injection;

pub struct NewProject {
    pub name: Option<String>,
    pub path: Option<String>,
}

impl Injection for NewProject {
    fn build_injection(injector: &mut anthill_di::Injector) -> Result<Self, anthill_di::DiError> {
        Ok(Self{name: None, path: None})
    }
}