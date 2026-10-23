use iced::widget::{button, column, row, space, text};
use iced::{Element, Fill};

use crate::app::Message;
use crate::components::dialog;
use crate::theme::{self, HEADING, MUTED};
use crate::update::UpdateInfo;

pub fn view(info: &UpdateInfo) -> Element<'_, Message> {
    column![
        dialog::header("发现新版本", None),
        space().height(18),
        text(format!("v{}", info.version)).size(26).font(HEADING),
        space().height(6),
        text(concat!("当前版本 v", env!("CARGO_PKG_VERSION")))
            .size(12)
            .color(MUTED),
        space().height(18),
        text("前往 GitHub 下载并安装更新。").size(12).color(MUTED),
        space().height(22),
        row![
            button(text("稍后").height(Fill).width(Fill).center())
                .on_press(Message::OverlayOpened(None))
                .style(theme::quiet_button)
                .height(36)
                .width(72)
                .padding(0),
            button(text("前往下载").height(Fill).width(Fill).center())
                .on_press(Message::OpenDownload)
                .style(theme::primary_button)
                .height(36)
                .width(Fill)
                .padding(0),
        ]
        .spacing(10),
    ]
    .into()
}
