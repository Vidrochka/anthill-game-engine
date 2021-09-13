use anthill_di::{Container, DiError, builders::ContainerBuilder};
use anthill_window_lib::{config::window_system_config::{DrawApi, WindowSystemConfig}, gui::{mvc::TView, screen::TScreen}, window::window_system::WindowSystem};

use crate::{
    config::{
        CoreConfig, 
        LoggerConfig
    }, 
    core::{
        engine_core::EngineCore, 
        state::CoreState
    }, 
    gui::{
        controllers::new_project::NewProjectController, 
        models::{
            new_project::NewProject
        }, 
        screen::screen::Screen, views::new_project::NewProjectView
    }, 
    utils::{
        logger_builder::LoggerBuilder, time::TimeTracker
    }
};

pub struct ContainersBuilder {}

impl ContainersBuilder {
    pub fn construct() -> Vec<Container> {
        let mut containers = vec![];
        containers.append(&mut ContainersBuilder::construct_core_parts());
        containers.append(&mut ContainersBuilder::construct_configs());
        containers.append(&mut ContainersBuilder::construct_models());
        containers.append(&mut ContainersBuilder::construct_controller());
        containers.append(&mut ContainersBuilder::construct_view());
        containers.append(&mut ContainersBuilder::construct_screen());
        containers.append(&mut ContainersBuilder::construct_utils());
        containers.append(&mut ContainersBuilder::construct_window_system());

        containers
    }

    pub fn construct_core_parts() -> Vec<Container> {
        vec![
            ContainerBuilder::bind_type::<EngineCore>().build(),
            ContainerBuilder::bind_type::<CoreState>().build(),
        ]
    }

    pub fn construct_configs() -> Vec<Container> {
        vec![
            ContainerBuilder::bind_unconfigured_type::<LoggerConfig>()
                .build_with_constructor(|_| -> _ {LoggerConfig::load().map_err(|e| DiError::CustomInjectTimeError(e))}),
            ContainerBuilder::bind_unconfigured_type::<CoreConfig>()
                .build_with_constructor(|_| -> _ {CoreConfig::load().map_err(|e| DiError::CustomInjectTimeError(e))}),
        ]
    }

    pub fn construct_models() -> Vec<Container> {
        vec![
            ContainerBuilder::bind_type::<NewProject>().build()
        ]
    }

    pub fn construct_view() -> Vec<Container> {
        vec![
            ContainerBuilder::bind_interface::<dyn TView<NewProjectController,NewProject>, NewProjectView>().build(),
        ]
    }

    pub fn construct_controller() -> Vec<Container> {
        vec![
            ContainerBuilder::bind_type::<NewProjectController>().build(),
        ]
    }

    pub fn construct_screen() -> Vec<Container> {
        vec![
            ContainerBuilder::bind_interface::<dyn TScreen,Screen>().build(),
        ]
    }

    pub fn construct_utils() -> Vec<Container> {
        vec![
            ContainerBuilder::bind_type::<TimeTracker>().build(),
            ContainerBuilder::bind_unconfigured_type::<log4rs::Handle>()
                .build_with_constructor(|injector| -> _ { LoggerBuilder::build(injector.get_new_instance()?).map_err(|e| DiError::CustomInjectTimeError(e)) }),
        ]
    }

    pub fn construct_window_system() -> Vec<Container> {
        vec![
            ContainerBuilder::bind_type::<WindowSystem>().build(),
            ContainerBuilder::bind_unconfigured_type::<WindowSystemConfig>().build_with_constructor(|_| -> _ {
                Ok(WindowSystemConfig{draw_api: DrawApi::OpenGL})
            }),
        ]
    }
}