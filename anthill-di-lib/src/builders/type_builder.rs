
use std::{any::{Any, TypeId}, marker::PhantomData};

use crate::{container::Container, injection::Injection, injector::Injector};

pub struct TypeBuilder<TType> where TType: Injection + 'static {
    pub phantom: PhantomData<TType>
}

impl<TType> TypeBuilder<TType> where TType: Injection + 'static {
    pub fn build(self) -> Container where TType: Injection + 'static, {
        let constructor: Box<dyn Fn(&mut Injector) -> Box<dyn Any>> = Box::new(|injector: &mut Injector| -> Box<dyn Any> {
            Box::new(TType::build_injection(injector))
        });

        Container {
            interface_type_id: TypeId::of::<TType>(),
            constructor: Some(constructor),
            instance: None
        }
    }
}