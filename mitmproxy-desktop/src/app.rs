use crate::{
    cert_manager::CertManager,
    proxy::{ProxyManager, ProxyState},
};
use anyhow::Result;
use iced::{
    widget::{button, column, container, row, text},
    Application, Element, Renderer, Theme,
};
use tracing::*;

#[derive(Debug, Clone)]
pub enum Message {
    GenerateCA,
    StartProxy(u16),
    StopProxy,
    ProxyStateChanged(ProxyState),
    Error(String),
}

pub struct MitmproxyDesktop {
    cert_manager: CertManager,
    proxy_manager: ProxyManager,
    proxy_state: ProxyState,
    error_message: Option<String>,
    install_instructions: Option<String>,
}

impl MitmproxyDesktop {
    pub fn new() -> Result<Self> {
        Ok(Self {
            cert_manager: CertManager::new()?,
            proxy_manager: ProxyManager::new(),
            proxy_state: ProxyState::Stopped,
            error_message: None,
            install_instructions: None,
        })
    }
}

impl iced::Application for MitmproxyDesktop {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, iced::Command<Message>) {
        match Self::new() {
            Ok(app) => (app, iced::Command::none()),
            Err(e) => {
                error!("Failed to initialize app: {}", e);
                panic!("Failed to initialize app: {}", e);
            }
        }
    }

    fn title(&self) -> String {
        String::from("Mitmproxy Desktop")
    }

    fn update(&mut self, message: Message) -> iced::Command<Message> {
        match message {
            Message::GenerateCA => {
                match self.cert_manager.generate_ca() {
                    Ok(()) => {
                        self.install_instructions =
                            Some(self.cert_manager.get_install_instructions());
                        self.error_message = None;
                    }
                    Err(e) => {
                        self.error_message = Some(format!("Failed to generate CA: {}", e));
                    }
                }
                iced::Command::none()
            }
            Message::StartProxy(port) => {
                if !self.cert_manager.has_ca() {
                    self.error_message = Some("Please generate CA certificate first".to_string());
                    return iced::Command::none();
                }

                let cert_path = self.cert_manager.get_cert_path().to_path_buf();
                let proxy_manager = self.proxy_manager.clone();

                iced::Command::perform(
                    async move {
                        proxy_manager
                            .start(port, Some(cert_path.to_string_lossy().into_owned()))
                            .await
                    },
                    |result| match result {
                        Ok(()) => Message::ProxyStateChanged(ProxyState::Running),
                        Err(e) => Message::Error(format!("Failed to start proxy: {}", e)),
                    },
                )
            }
            Message::StopProxy => {
                let proxy_manager = self.proxy_manager.clone();
                iced::Command::perform(async move { proxy_manager.stop().await }, |result| {
                    match result {
                        Ok(()) => Message::ProxyStateChanged(ProxyState::Stopped),
                        Err(e) => Message::Error(format!("Failed to stop proxy: {}", e)),
                    }
                })
            }
            Message::ProxyStateChanged(state) => {
                self.proxy_state = state;
                self.error_message = None;
                iced::Command::none()
            }
            Message::Error(e) => {
                self.error_message = Some(e);
                iced::Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let title = text("Mitmproxy Desktop").size(32);
        let cert_status = if !self.cert_manager.has_ca() {
            column![button("Generate CA Certificate").on_press(Message::GenerateCA)]
        } else {
            column![row![
                text("CA Certificate installed ✓").size(16),
                button("Regenerate").on_press(Message::GenerateCA)
            ]
            .spacing(10)]
        };

        let mut content = column![title, cert_status].spacing(20);

        if let Some(instructions) = &self.install_instructions {
            content = content.push(
                container(text(instructions).size(14))
                    .padding(10)
                    .style(iced::theme::Container::Box),
            );
        }

        if let Some(error) = &self.error_message {
            content = content.push(
                container(text(error).size(14).style(iced::theme::Text::Color(
                    iced::Color::from_rgb(0.8, 0.0, 0.0),
                )))
                .padding(10),
            );
        }

        match self.proxy_state {
            ProxyState::Stopped => {
                content = content
                    .push(button("Start Proxy on port 8080").on_press(Message::StartProxy(8080)));
            }
            ProxyState::Running => {
                content = content.push(button("Stop Proxy").on_press(Message::StopProxy));
            }
            ProxyState::Error => {
                content = content.push(text("Proxy Error").size(14));
            }
        }

        container(content)
            .padding(20)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .into()
    }
}
