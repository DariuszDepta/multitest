use crate::api::{MultiTestApi, MultiTestMockApi};

pub struct Chain {
    /// API used in the chain.
    api: Box<dyn MultiTestApi>,
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
            api: Box::<MultiTestMockApi>::default(),
        }
    }

    pub fn with_api(mut self, api: impl MultiTestApi + 'static) -> Self {
        self.api = Box::new(api);
        self
    }

    /// Returns a reference to API configured in [Chain].
    pub fn api(&self) -> &dyn MultiTestApi {
        self.api.as_ref()
    }
}
