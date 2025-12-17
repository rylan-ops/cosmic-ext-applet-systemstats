mod app;
mod monitors;

use app::SystemStats;

fn main() -> cosmic::iced::Result {
    cosmic::applet::run::<SystemStats>(())
}
