use std::{any::{TypeId, type_name}, collections::HashMap, sync::{Arc, Mutex}};

use crate::container::Container;

pub struct Injector {
    containers: HashMap<TypeId, Container>
}

impl Injector {
    pub fn new(contsiners: Vec<Container>) -> Arc<Mutex<Self>> {
        let injector = Arc::new(Mutex::new(Self{containers: HashMap::new()}));

        let mut containers_map: HashMap<TypeId, Container> = contsiners.into_iter().map(|c| { (c.interface_type_id.clone(), c) }).collect();
        
        containers_map.insert(
            TypeId::of::<Self>(), 
            Container {
                interface_type_id: TypeId::of::<Self>(),
                //implementation_type_id: Some(TypeId::of::<Self>()),
                constructor: None,
                instance: Some(Box::new(Arc::clone(&injector))),
            }
        );

        for (type_id, container) in &containers_map {
            println!("[injector::new] {:?}, {:?}", type_id, container.interface_type_id/* , container.implementation_type_id */)
        }

        injector.lock().unwrap().containers = containers_map;

        injector
    }

    pub fn get_singletone<TType>(&mut self) -> Arc<Mutex<TType>> where TType: 'static {
        println!("[injector::build_singletone] {:?}, {:?}", type_name::<TType>(), TypeId::of::<TType>());
        match self.containers.remove(&TypeId::of::<TType>()) {
            Some(mut container) => {
                let obj = container.build_singletone::<TType>(self);
                self.containers.insert(container.interface_type_id.clone(), container);
                obj
                
            },
            None => panic!("[injector::build_singletone] container not found {}", type_name::<TType>()),
        }
    }

    //fn get_

    pub fn get_new_instance<TType>(&mut self) -> TType where TType: 'static {
        println!("[injector::build_new_instance] {:?}, {:?}", type_name::<TType>(), TypeId::of::<TType>());
        match self.containers.remove(&TypeId::of::<TType>()) {
            Some(container) => {
                let obj = container.build_new_instance::<TType>(self);
                self.containers.insert(container.interface_type_id.clone(), container);
                obj
                
            },
            None => panic!("[injector::build_new_instance] container not found {}", type_name::<TType>()),
        }
    }
}