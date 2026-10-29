use std::time::{Duration, Instant};

use iced::keyboard::key::Named;
use iced::keyboard::{self, Key};
use iced::widget::operation;
use iced::{Element, Size, Subscription, Task, window};
use tray_icon::TrayIcon;

use crate::config::{self, Credentials, Settings};
use crate::network::LoginStatus;
use crate::state::{OnlineInfo, Session};
use crate::update::UpdateInfo;
use crate::{autostart, network, tray, update, views};

const POLL_INTERVAL: Duration = Duration::from_secs(3);
const APP_ICON: &[u8] = include_bytes!("../assets/icons/icon.png");
const REPOSITORY_URL: &str = "https://github.com/bitoolkit/biternet";

pub struct App {
    pub credentials: Credentials,
    pub settings: Settings,
    pub session: Session,
    pub overlay: Option<Overlay>,
    pub notice: Option<Notice>,
    pub update_info: Option<UpdateInfo>,

    checking_status: bool,
    manual_logout: bool,
    window_id: Option<window::Id>,
    tray: Option<TrayIcon>,
}

pub struct Notice {
    pub title: &'static str,
    pub detail: Option<String>,
    expires_at: Option<Instant>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Overlay {
    Settings,
    Update,
    Notice,
}

#[derive(Clone, Debug)]
pub enum Message {
    Started { silent: bool },
    WindowOpened(window::Id),
    WindowClosed(window::Id),
    CloseRequested(window::Id),
    CloseWindow,
    Tray(tray::Action),

    UsernameChanged(String),
    PasswordChanged(String),
    RememberPasswordChanged(bool),
    AutoLoginChanged(bool),

    Login,
    LoginFinished(Result<OnlineInfo, String>),
    Logout,
    LogoutFinished(Result<(), String>),

    RefreshStatus,
    StatusChecked(Result<LoginStatus, String>),

    OverlayOpened(Option<Overlay>),
    AutoStartChanged(bool),
    SilentStartChanged(bool),

    DismissNotice,
    NoticeExpired(Instant),

    UpdateChecked(Option<UpdateInfo>),
    OpenDownload,
    OpenRepository,
    LinkOpened(Result<(), String>),

    FocusNext,
    FocusPrevious,
    Escape,
}

impl App {
    pub fn new() -> (Self, Task<Message>) {
        let app = Self {
            credentials: config::credentials::load(),
            settings: config::settings::load(),
            session: Session::Initializing,
            overlay: None,
            notice: None,
            update_info: None,
            checking_status: true,
            manual_logout: false,
            window_id: None,
            tray: None,
        };

        let task = Task::batch([
            Task::done(Message::Started {
                silent: std::env::args().any(|arg| arg == "--silent"),
            }),
            check_status(),
            Task::perform(update::check_for_update(), |result| {
                Message::UpdateChecked(result.ok().flatten())
            }),
        ]);

        (app, task)
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Started { silent } => {
                // Native tray icons must be created on the running UI event
                // loop.
                match tray::create() {
                    Ok(tray) => self.tray = Some(tray),
                    Err(error) => {
                        let notice = self.show_notice("无法创建托盘", Some(error.to_string()));

                        return Task::batch([notice, self.show_window()]);
                    }
                }

                if self.settings.auto_start
                    && let Err(error) = autostart::sync(&self.settings)
                {
                    let notice = self.show_notice("无法更新开机启动项", Some(error.to_string()));

                    return Task::batch([notice, self.show_window()]);
                }

                if !silent {
                    return self.show_window();
                }
            }

            Message::WindowOpened(id) => {
                assert_eq!(Some(id), self.window_id);

                return window::gain_focus(id);
            }
            Message::WindowClosed(id) => {
                if self.window_id == Some(id) {
                    self.window_id = None;
                }
            }
            Message::CloseRequested(id) => {
                self.window_id = None;
                self.overlay = None;

                return if self.tray.is_some() {
                    // Wayland cannot hide an existing window. Keep the daemon
                    // and session alive, and recreate the window on tray
                    // activation.
                    window::close(id)
                } else {
                    iced::exit()
                };
            }
            Message::CloseWindow => {
                if let Some(id) = self.window_id {
                    return Task::done(Message::CloseRequested(id));
                }
            }

            Message::Tray(tray::Action::ShowWindow) => return self.show_window(),
            Message::Tray(tray::Action::Exit) => return iced::exit(),

            Message::UsernameChanged(value) => self.credentials.username = value,
            Message::PasswordChanged(value) => self.credentials.password = value,
            Message::RememberPasswordChanged(remember) => {
                self.credentials.remember_password = remember;

                if !remember {
                    self.credentials.auto_login = false;
                }

                config::credentials::save(&self.credentials);
            }
            Message::AutoLoginChanged(automatic) => {
                self.credentials.auto_login = automatic;

                if automatic {
                    self.credentials.remember_password = true;
                }

                config::credentials::save(&self.credentials);
            }

            Message::Login => {
                if self.can_login() {
                    return self.login(false);
                }
            }
            Message::LoginFinished(result) => {
                let Session::LoggingIn { automatic } = self.session else {
                    panic!("login result without an active login");
                };

                match result {
                    Ok(info) => {
                        config::credentials::save(&self.credentials);
                        self.session = Session::Online(info);

                        if !automatic {
                            return self.show_notice("登录成功", None);
                        }
                    }
                    Err(error) => {
                        self.session = Session::Offline;

                        if !automatic {
                            return self.show_notice("登录失败", Some(error));
                        }
                    }
                }
            }

            Message::Logout => {
                if self.can_logout() {
                    return self.logout();
                }
            }
            Message::LogoutFinished(result) => {
                let Session::LoggingOut(info) = &self.session else {
                    panic!("logout result without an active logout");
                };

                match result {
                    Ok(()) => {
                        self.session = Session::Offline;

                        return self.show_notice("已注销", None);
                    }
                    Err(error) => {
                        self.session = Session::Online(info.clone());

                        return self.show_notice("注销失败", Some(error));
                    }
                }
            }

            Message::RefreshStatus => {
                if !self.checking_status && !self.session.is_busy() {
                    self.checking_status = true;

                    return check_status();
                }
            }
            Message::StatusChecked(result) => {
                assert!(
                    self.checking_status,
                    "status result without an active check"
                );
                self.checking_status = false;

                // Queue authentication behind the active status request so
                // polling does not disable or flicker the idle controls.
                match self.session {
                    Session::LoggingIn { automatic } => return self.login(automatic),
                    Session::LoggingOut(_) => return self.logout(),
                    _ => {}
                }

                match result {
                    Ok(LoginStatus::Online(info)) => {
                        self.session = Session::Online(info);
                        self.manual_logout = false;
                    }
                    Ok(LoginStatus::Offline(_)) | Err(_) => {
                        self.session = Session::Offline;

                        if self.credentials.auto_login
                            && self.credentials.can_login()
                            && !self.manual_logout
                        {
                            return self.login(true);
                        }
                    }
                }
            }

            Message::OverlayOpened(overlay) => {
                self.overlay = overlay;

                if overlay == Some(Overlay::Settings) {
                    self.notice = None;
                }
            }
            Message::AutoStartChanged(enabled) => {
                let next = Settings {
                    auto_start: enabled,
                    silent_start: enabled && self.settings.silent_start,
                };

                return self.save_settings(next);
            }
            Message::SilentStartChanged(enabled) => {
                let next = Settings {
                    silent_start: self.settings.auto_start && enabled,
                    ..self.settings.clone()
                };

                return self.save_settings(next);
            }

            Message::DismissNotice => {
                self.notice = None;

                if self.overlay == Some(Overlay::Notice) {
                    self.overlay = None;
                }
            }
            Message::NoticeExpired(deadline) => {
                if self
                    .notice
                    .as_ref()
                    .is_some_and(|notice| notice.expires_at == Some(deadline))
                {
                    self.notice = None;
                }
            }

            Message::UpdateChecked(info) => self.update_info = info,
            Message::OpenDownload => {
                if let Some(info) = &self.update_info {
                    return open_link(info.html_url.clone());
                }
            }
            Message::OpenRepository => return open_link(REPOSITORY_URL.into()),
            Message::LinkOpened(Err(error)) => {
                self.overlay = None;

                return self.show_notice("无法打开浏览器", Some(error));
            }

            Message::FocusNext if self.overlay.is_none() => return operation::focus_next(),
            Message::FocusPrevious if self.overlay.is_none() => return operation::focus_previous(),
            Message::LinkOpened(Ok(())) | Message::FocusNext | Message::FocusPrevious => {}
            Message::Escape => {
                if self.overlay.take().is_none() {
                    return Task::done(Message::CloseWindow);
                }
            }
        }

        Task::none()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::batch([
            iced::time::every(POLL_INTERVAL).map(|_| Message::RefreshStatus),
            window::close_requests().map(Message::CloseRequested),
            window::close_events().map(Message::WindowClosed),
            Subscription::run(tray::events).map(Message::Tray),
            keyboard::listen().filter_map(|event| match event {
                keyboard::Event::KeyPressed { key, modifiers, .. } => match key {
                    Key::Character(value) if value.as_str() == "," && modifiers.command() => {
                        Some(Message::OverlayOpened(Some(Overlay::Settings)))
                    }
                    Key::Named(Named::Tab) if modifiers.shift() => Some(Message::FocusPrevious),
                    Key::Named(Named::Tab) => Some(Message::FocusNext),
                    Key::Named(Named::Escape) => Some(Message::Escape),
                    _ => None,
                },
                _ => None,
            }),
        ])
    }

    pub fn view(&self, _window: window::Id) -> Element<'_, Message> {
        views::view(self)
    }

    pub fn can_login(&self) -> bool {
        self.overlay.is_none()
            && !self.session.is_busy()
            && self.session.online().is_none()
            && self.credentials.can_login()
    }

    pub const fn can_logout(&self) -> bool {
        self.overlay.is_none() && matches!(self.session, Session::Online(_))
    }

    fn login(&mut self, automatic: bool) -> Task<Message> {
        self.session = Session::LoggingIn { automatic };
        self.manual_logout = false;

        if !automatic {
            self.notice = None;
        }

        if self.checking_status {
            return Task::none();
        }

        Task::perform(
            network::login(
                self.credentials.username.clone(),
                self.credentials.password.clone(),
            ),
            |result| Message::LoginFinished(result.map_err(|error| error.to_string())),
        )
    }

    fn logout(&mut self) -> Task<Message> {
        if let Session::Online(info) = &self.session {
            self.session = Session::LoggingOut(info.clone());
            self.manual_logout = true;
            self.notice = None;
        }

        if self.checking_status {
            return Task::none();
        }

        let Session::LoggingOut(info) = &self.session else {
            panic!("logout requires an online session");
        };

        Task::perform(network::logout(info.username.clone()), |result| {
            Message::LogoutFinished(result.map_err(|error| error.to_string()))
        })
    }

    fn show_window(&mut self) -> Task<Message> {
        if let Some(id) = self.window_id {
            return window::minimize(id, false).chain(window::gain_focus(id));
        }

        let icon = image::load_from_memory(APP_ICON).unwrap().into_rgba8();
        let icon = window::icon::from_rgba(icon.to_vec(), icon.width(), icon.height()).unwrap();

        let size = Size::new(252.0, 332.0);
        let (id, open) = window::open(window::Settings {
            size,
            min_size: Some(size),
            max_size: Some(size),
            resizable: false,
            position: window::Position::Centered,
            icon: Some(icon),
            exit_on_close_request: false,
            ..Default::default()
        });
        self.window_id = Some(id);

        open.map(Message::WindowOpened)
    }

    fn show_notice(&mut self, title: &'static str, detail: Option<String>) -> Task<Message> {
        if self.overlay == Some(Overlay::Notice) {
            self.overlay = None;
        }

        let expires_at = detail
            .is_none()
            .then(|| Instant::now() + Duration::from_millis(2_400));
        self.notice = Some(Notice {
            title,
            detail,
            expires_at,
        });

        let Some(expires_at) = expires_at else {
            return Task::none();
        };

        Task::perform(
            async move {
                tokio::time::sleep_until(tokio::time::Instant::from_std(expires_at)).await;

                expires_at
            },
            Message::NoticeExpired,
        )
    }

    fn save_settings(&mut self, next: Settings) -> Task<Message> {
        match autostart::sync(&next) {
            Ok(()) => {
                config::settings::save(&next);
                self.settings = next;

                self.show_notice("设置已保存", None)
            }
            Err(error) => self.show_notice("无法保存设置", Some(error.to_string())),
        }
    }
}

fn check_status() -> Task<Message> {
    Task::perform(network::check_status(), |result| {
        Message::StatusChecked(result.map_err(|error| error.to_string()))
    })
}

fn open_link(url: String) -> Task<Message> {
    Task::perform(
        async move {
            tokio::task::spawn_blocking(move || open::that(url))
                .await
                .unwrap()
                .map_err(|error| error.to_string())
        },
        Message::LinkOpened,
    )
}
