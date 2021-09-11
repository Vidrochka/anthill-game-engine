use std::marker::PhantomData;

use crate::injection::Injection;

use super::{type_builder::TypeBuilder, interface_builder::InterfaceBuilder};

pub struct ContainerBuilder {
}

impl ContainerBuilder {
    pub fn bind_interface<TInterface>() -> InterfaceBuilder<TInterface> where TInterface: 'static + ?Sized,
    {
        InterfaceBuilder{phantom_interface: PhantomData}
    }

    pub fn bind_type<TType>() -> TypeBuilder<TType> where TType: Injection + 'static  {
        TypeBuilder{phantom: PhantomData}
    }
}