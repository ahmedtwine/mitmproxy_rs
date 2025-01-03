use anyhow::Result;
use iced::{
    executor,
    widget::{button, column, container, row, text, text_input},
    Application, Command, Element, Length, Theme,
};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::config::Config;
use crate::proxy::{ProxyManager, ProxyState};

#[derive(Debug, Clone)]
pub enum Message {
    StartProxy,
    StopProxy,
    SelectCertificate,
    CertificateSelected(String),
    PortChanged(String),
    ProxyStateChanged(ProxyState),
}

pub struct MitmproxyDesktop {
    proxy_manager: Arc<Mutex<ProxyManager>>,
    config: Config,
    proxy_state: ProxyState,
    port: String,
    certificate_path: Option<String>,
}

impl Application for MitmproxyDesktop {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let config = Config::load().unwrap_or_default();
        (
            Self {
                proxy_manager: Arc::new(Mutex::new(ProxyManager::new())),
                config,
                proxy_state: ProxyState::Stopped,
                port: "8080".to_string(),
                certificate_path: None,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Mitmproxy Desktop")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::StartProxy => {
                let proxy_manager = Arc::clone(&self.proxy_manager);
                let port = self.port.clone();
                let cert_path = self.certificate_path.clone();

                Command::perform(
                    async move {
                        let mut manager = proxy_manager.lock().await;
                        manager
                            .start(port.parse().unwrap_or(8080), cert_path)
                            .await?;
                        Ok(ProxyState::Running)
                    },
                    |result: Result<ProxyState>| {
                        Message::ProxyStateChanged(result.unwrap_or(ProxyState::Error))
                    },
                )
            }
            Message::StopProxy => {
                let proxy_manager = Arc::clone(&self.proxy_manager);
                Command::perform(
                    async move {
                        let mut manager = proxy_manager.lock().await;
                        manager.stop().await?;
                        Ok(ProxyState::Stopped)
                    },
                    |result: Result<ProxyState>| {
                        Message::ProxyStateChanged(result.unwrap_or(ProxyState::Error))
                    },
                )
            }
            Message::SelectCertificate => Command::perform(
                async {
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("Certificate", &["cer", "pem"])
                        .pick_file()
                    {
                        path.to_str().map(String::from)
                    } else {
                        None
                    }
                },
                |path| Message::CertificateSelected(path.unwrap_or_default()),
            ),
            Message::CertificateSelected(path) => {
                self.certificate_path = Some(path);
                Command::none()
            }
            Message::PortChanged(port) => {
                self.port = port;
                Command::none()
            }
            Message::ProxyStateChanged(state) => {
                self.proxy_state = state;
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let controls = row![
            text_input("Port", &self.port)
                .on_input(Message::PortChanged)
                .padding(10),
            button("Select Certificate")
                .on_press(Message::SelectCertificate)
                .padding(10),
            match self.proxy_state {
                ProxyState::Stopped => button("Start Proxy").on_press(Message::StartProxy),
                ProxyState::Running => button("Stop Proxy").on_press(Message::StopProxy),
                ProxyState::Error => button("Retry").on_press(Message::StartProxy),
            }
            .padding(10)
        ]
        .spacing(20)
        .padding(20);

        let status = text(format!("Status: {:?}", self.proxy_state)).size(20);
        let cert_status = text(format!(
            "Certificate: {}",
            self.certificate_path.as_deref().unwrap_or("Not selected")
        ))
        .size(16);

        let content = column![controls, status, cert_status]
            .spacing(20)
            .padding(20);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
