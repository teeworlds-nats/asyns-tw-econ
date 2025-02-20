use std::net::SocketAddr;
use crate::raw::EconRaw;

#[derive(Default)]
pub struct Econ {
    raw: Option<EconRaw>,
}

impl Econ {
    pub fn new() -> Self {
        Self::default()
    }

    /// Connects to given address
    pub async fn connect(&mut self, address: impl Into<SocketAddr>) -> std::io::Result<()> {
        self.raw = Some(EconRaw::connect(address, 2048, 5).await?);
        Ok(())
    }

    pub async fn disconnect(&mut self) -> std::io::Result<()> {
        assert!(
            self.raw.is_some(),
            "you can't disconnect without being connected"
        );

        let raw = self.raw.as_mut().unwrap();
        raw.disconnect().await
    }

    /// Tries to authenticate, returns `false` if password is incorrect
    pub async fn try_auth(&mut self, password: impl Into<String>) -> std::io::Result<bool> {
        assert!(
            self.raw.is_some(),
            "you can't authenticate without being connected"
        );

        let raw = self.raw.as_mut().unwrap();
        raw.auth(password.into().as_str()).await
    }

    /// Change auth message
    pub fn set_auth_message<T: ToString>(&mut self, auth_message: T) {
        self.raw.as_mut().unwrap().set_auth_message(auth_message.to_string());
    }

    /// Asynchronous *write* operation, sends line to socket
    pub async fn send_line(&mut self, line: impl Into<String>) -> std::io::Result<()> {
        assert!(
            self.raw.is_some(),
            "you can't send commands without being connected"
        );

        let raw = self.raw.as_mut().unwrap();

        assert!(
            raw.is_authed(),
            "you can't send commands without being authed"
        );

        raw.send(line.into().as_str()).await
    }

    /// Asynchronous *read* operation, reads to buffer and appends to inner line buffer
    /// if fetch set to `true`, otherwise returns popped line from line buffer
    /// with no another operation
    pub async fn recv_line(&mut self, fetch: bool) -> std::io::Result<Option<String>> {
        assert!(
            self.raw.is_some(),
            "you can't fetch lines without being connected"
        );

        let raw = self.raw.as_mut().unwrap();

        if fetch {
            assert!(
                raw.is_authed(),
                "you can't fetch lines without being authed"
            );

            raw.read().await?;
        }

        Ok(raw.pop_line())
    }
}