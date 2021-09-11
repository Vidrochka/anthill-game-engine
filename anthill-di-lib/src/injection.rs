use crate::injector::Injector;

pub trait Injection {
    fn build_injection(injector: &mut Injector) -> Self;
}

impl<T> Injection for Box<T> where T: Injection {
    fn build_injection(injector: &mut Injector) -> Self {
        Box::new(T::build_injection(injector))
    }
}