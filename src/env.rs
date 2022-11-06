pub enum Environment {
    Client,
    Server,
}

impl Environment {
    pub fn is_client(&self) -> bool {
        matches!(self, Environment::Client)
    }

    pub fn is_server(&self) -> bool {
        matches!(self, Environment::Server)
    }
}
