use std::{any::{Any, TypeId}, marker::{PhantomData, Unsize}};

use crate::{container::Container, injection::Injection, injector::Injector};

pub struct InterfaceBuilder<TInterface> where TInterface: 'static + ?Sized
{
    pub phantom_interface: PhantomData<TInterface>,
}

impl<TInterface> InterfaceBuilder<TInterface> where TInterface: 'static + ?Sized
{
    pub fn to_type<TType>(self) -> Container where TType: Injection + Unsize<TInterface> + 'static
    {
        let constructor: Box<dyn Fn(&mut Injector) -> Box<dyn Any>> = Box::new(|injector: &mut Injector| -> Box<dyn Any> {
            let interface: Box<TInterface> = Box::new(TType::build_injection(injector)) as Box<TInterface>;
            Box::new(interface)
        });

        Container {
            interface_type_id: TypeId::of::<Box<TInterface>>(),
            constructor: Some(constructor),
            instance: None
        }
    }
}