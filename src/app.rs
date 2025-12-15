use cosmic::app::{Core, Task};
use cosmic::iced::{Alignment, Limits, Subscription};
use cosmic::iced::time;
use cosmic::iced_core::text::Wrapping;
use cosmic::iced_widget::Row;
use cosmic::widget::{autosize, text};
use cosmic::Element;
use sysinfo::System;
use std::time::Duration;

const ID: &str = "com.github.rylan-x.systemstats";

/// Main applet struct
pub struct SystemStats {
    core: Core,
    system: System,
    cpu_usage: f32,
    memory_used_gb: f32,
    memory_total_gb: f32,
}

/// Messages the applet can receive
#[derive(Debug, Clone)]
pub enum Message {
    Tick,
}

impl cosmic::Application for SystemStats {
    type Executor = cosmic::executor::Default;
    type Flags = ();
    type Message = Message;
    const APP_ID: &'static str = ID;

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    fn init(core: Core, _flags: Self::Flags) -> (Self, Task<Self::Message>) {
        let mut system = System::new_all();
        system.refresh_all();

        let cpu_usage = system.global_cpu_usage();
        let memory_total = system.total_memory() as f32 / 1_073_741_824.0;
        let memory_used = system.used_memory() as f32 / 1_073_741_824.0;

        let app = SystemStats {
            core,
            system,
            cpu_usage,
            memory_used_gb: memory_used,
            memory_total_gb: memory_total,
        };
        (app, Task::none())
    }

    fn update(&mut self, message: Self::Message) -> Task<Self::Message> {
        match message {
            Message::Tick => {
                self.system.refresh_cpu_usage();
                self.system.refresh_memory();

                self.cpu_usage = self.system.global_cpu_usage();
                self.memory_used_gb = self.system.used_memory() as f32 / 1_073_741_824.0;
                self.memory_total_gb = self.system.total_memory() as f32 / 1_073_741_824.0;
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let stats_text = format!(
            "CPU: {:.0}% | RAM: {:.1}GB",
            self.cpu_usage,
            self.memory_used_gb
        );

        let elements = vec![
            text(stats_text)
                .wrapping(Wrapping::None)
                .into()
        ];

        let content = Row::from_vec(elements)
            .padding([0, 8])
            .align_y(Alignment::Center);

        let limits = Limits::NONE
            .max_width(400.0)
            .min_height(1.0)
            .max_height(128.0);

        autosize::autosize(content, cosmic::widget::Id::unique())
            .limits(limits)
            .into()
    }

    fn style(&self) -> Option<cosmic::iced_runtime::Appearance> {
        Some(cosmic::applet::style())
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        time::every(Duration::from_secs(1)).map(|_| Message::Tick)
    }
}
