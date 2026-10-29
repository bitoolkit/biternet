use iced::widget::{button, column, container, row, scrollable, space, text};
use iced::{Alignment, Element, Fill, Padding};

use crate::app::{Message, Notice, Overlay};
use crate::components::dialog;
use crate::theme;

pub fn bar(notice: Option<&Notice>) -> Element<'_, Message> {
    let Some(notice) = notice else {
        return space().into();
    };

    let is_error = notice.detail.is_some();
    let color = if is_error {
        text::danger
    } else {
        text::success
    };

    let mut line = row![
        text(notice.title).size(11).style(color),
        space::horizontal()
    ]
    .align_y(Alignment::Center)
    .spacing(6);

    if is_error {
        line = line.push(
            button(text("详情").size(11).height(Fill).center())
                .on_press(Message::OverlayOpened(Some(Overlay::Notice)))
                .style(theme::quiet_button)
                .height(28)
                .padding([0, 4]),
        );
    }

    line.push(
        button(text("×").size(17).height(Fill).width(Fill).center())
            .on_press(Message::DismissNotice)
            .style(theme::quiet_button)
            .height(28)
            .width(24)
            .padding(0),
    )
    .into()
}

pub fn details(notice: &Notice) -> Element<'_, Message> {
    let detail = notice
        .detail
        .as_ref()
        .expect("notice dialog requires details");

    column![
        dialog::header(notice.title, None),
        space().height(16),
        container(
            scrollable(
                container(
                    text(detail)
                        .size(13)
                        .wrapping(text::Wrapping::WordOrGlyph)
                        .width(Fill)
                )
                .padding(Padding {
                    right: 14.0,
                    ..Padding::new(0.0)
                }),
            )
            .height(iced::Shrink)
            .width(Fill),
        )
        .max_height(144),
        space().height(20),
        button(text("知道了").height(Fill).width(Fill).center())
            .on_press(Message::DismissNotice)
            .style(theme::primary_button)
            .height(36)
            .width(Fill)
            .padding(0),
    ]
    .into()
}
