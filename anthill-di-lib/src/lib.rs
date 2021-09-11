#![feature(type_name_of_val)]
#![feature(allocator_api)]
#![feature(unsize)]

pub mod container;
pub mod injector;
pub mod injection;
pub mod builders;

#[cfg(test)]
mod tests {
    use crate::{builders::builder::ContainerBuilder, injection::Injection, injector::Injector};

    struct TestStruct1 {
        pub s: String,
    }

    impl TTestGetStr for TestStruct1 {
        fn get_str(&self) -> String {
            self.s.clone()
        }
    }

    impl Injection for TestStruct1 {
        fn build_injection(injector: &mut crate::injector::Injector) -> Self {
            Self { s: "test".to_string() }
        }
    }

    struct TestStruct2 {
        pub t1: TestStruct1,
    }

    impl Injection for TestStruct2 {
        fn build_injection(injector: &mut crate::injector::Injector) -> Self {
            Self { t1: injector.get_new_instance::<TestStruct1>() }
        }
    }

    struct TestStruct3 {
        pub t1: Box<dyn TTestGetStr>,
    }

    impl Injection for TestStruct3 {
        fn build_injection(injector: &mut crate::injector::Injector) -> Self {
            Self { t1: injector.get_new_instance::<Box<dyn TTestGetStr>>() }
        }
    }

    #[test]
    fn one_simple_type_inject() {

        let containers = vec![
            ContainerBuilder::bind_type::<TestStruct1>().build(),
            ContainerBuilder::bind_type::<TestStruct2>().build(),
        ];

        let injector = Injector::new(containers);

        let obj = injector.lock().unwrap().get_new_instance::<TestStruct2>();

        assert_eq!(obj.t1.s, "test".to_string());
    }

    struct StructWithText {
    }



    #[test]
    fn one_interface_inject() {
        let containers = vec![
            ContainerBuilder::bind_interface::<dyn TTestGetStr>().to_type::<TestStruct1>(),
            ContainerBuilder::bind_type::<TestStruct3>().build(),
        ];

        let injector = Injector::new(containers);

        let obj = injector.lock().unwrap().get_new_instance::<TestStruct3>();

        assert_eq!(obj.t1.get_str(), "test".to_string());
    }
}
