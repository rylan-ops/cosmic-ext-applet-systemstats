use cosmic::app::{Core, Task};
use cosmic::iced::{Alignment, Length};
use cosmic::widget::{container, text};
use cosmic::Element;

const ID: &str = "com.github.rylan-x.systemstats";

/// Main applet struct
pub struct SystemStats {
    core: Core,
    // TODO: Add system monitoring state
    // cpu_usage: f32,
    // memory_usage: f32,
}

/// Messages the applet can receive
#[derive(Debug, Clone)]
pub enum Message {
    // TODO: Add monitoring update messages
    // Tick,
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
        let app = SystemStats {
            core,
            // TODO: Initialize monitoring state
        };
        (app, Task::none())
    }

    fn update(&mut self, message: Self::Message) -> Task<Self::Message> {
        match message {
            // TODO: Handle monitoring update messages
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        // TODO: Display real system stats
        container(text("Stats"))
            .padding([0, 8])
            .height(Length::Fill)
            .align_y(Alignment::Center)
            .into()
    }

    fn style(&self) -> Option<cosmic::iced_runtime::Appearance> {
        Some(cosmic::applet::style())
    }

    // TODO: Add subscription for periodic updates
    // fn subscription(&self) -> Subscription<Self::Message> {
    //     time::every(Duration::from_secs(1)).map(|_| Message::Tick)
    // }
}
