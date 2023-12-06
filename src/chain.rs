use cosmwasm_std::testing::MockApi;
use cosmwasm_std::Api;

pub struct Chain {
    /// [Api] used in the chain.
    api: Box<dyn Api>,
}

impl Default for Chain {
    /// Creates a [Chain] with default configuration.
    fn default() -> Self {
        Self::new()
    }
}

impl Chain {
    /// Creates a new instance of a [Chain].
    pub fn new() -> Self {
        Self {
            api: Box::<MockApi>::default(),
        }
    }

    pub fn with_api(mut self, api: impl Api + 'static) -> Self {
        self.api = Box::new(api);
        self
    }

    /// Returns a reference to [Api] configured for a [Chain].
    pub fn api(&self) -> &dyn Api {
        self.api.as_ref()
    }
}
