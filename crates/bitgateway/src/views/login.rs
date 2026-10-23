use iced::widget::{button, checkbox, column, row, space, text, text_input};
use iced::{Alignment, Element, Fill};

use crate::app::{App, Message};
use crate::state::Session;
use crate::theme::{self, MUTED};

pub fn view(app: &App) -> Element<'_, Message> {
    let editable = app.overlay.is_none() && !app.session.is_busy();
    let username = text_input("学号或用户名", &app.credentials.username)
        .on_input_maybe(editable.then_some(Message::UsernameChanged))
        .on_submit_maybe(app.can_login().then_some(Message::Login))
        .size(14)
        .padding([8, 10])
        .style(theme::input);

    let password = text_input("统一认证密码", &app.credentials.password)
        .secure(true)
        .on_input_maybe(editable.then_some(Message::PasswordChanged))
        .on_submit_maybe(app.can_login().then_some(Message::Login))
        .size(14)
        .padding([8, 10])
        .style(theme::input);

    let remember = checkbox(app.credentials.remember_password)
        .label("记住密码")
        .on_toggle_maybe(editable.then_some(Message::RememberPasswordChanged))
        .size(15)
        .spacing(7)
        .text_size(12)
        .style(theme::checkmark);

    let automatic = checkbox(app.credentials.auto_login)
        .label("自动登录")
        .on_toggle_maybe(editable.then_some(Message::AutoLoginChanged))
        .size(15)
        .spacing(7)
        .text_size(12)
        .style(theme::checkmark);

    let label = match app.session {
        Session::Initializing => "正在检查连接…",
        Session::LoggingIn { automatic: true } => "自动登录中…",
        Session::LoggingIn { automatic: false } => "登录中…",
        _ => "登录",
    };

    column![
        column![text("用户名").size(12).color(MUTED), username].spacing(5),
        space().height(12),
        column![text("密码").size(12).color(MUTED), password].spacing(5),
        space().height(12),
        row![remember, space::horizontal(), automatic].align_y(Alignment::Center),
        space::vertical(),
        button(text(label).height(Fill).width(Fill).center())
            .on_press_maybe(app.can_login().then_some(Message::Login))
            .style(theme::primary_button)
            .width(Fill)
            .height(34)
            .padding(0),
    ]
    .height(Fill)
    .into()
}
