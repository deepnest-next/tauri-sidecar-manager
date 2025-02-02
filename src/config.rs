#[derive(Debug, Clone)]
pub struct SidecarConfig {
    pub sidecar_name: String,
    pub args: Vec<String>,
    pub spawn_on_start: bool,
    pub working_directory: Option<String>,
}

impl SidecarConfig {
    pub fn builder() -> SidecarConfigBuilder {
        SidecarConfigBuilder::new()
    }

    pub fn args(&self) -> &[String] {
        &self.args
    }
}

#[derive(Default)]
pub struct SidecarConfigBuilder {
    sidecar_name: Option<String>,
    args: Vec<String>,
    spawn_on_start: Option<bool>,
    working_directory: Option<String>,
}

impl SidecarConfigBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn sidecar_name(mut self, name: impl Into<String>) -> Self {
        self.sidecar_name = Some(name.into());
        self
    }

    pub fn args<I, S>(mut self, args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.args = args.into_iter().map(|s| s.into()).collect();
        self
    }

    pub fn arg(mut self, arg: impl Into<String>) -> Self {
        self.args.push(arg.into());
        self
    }

    pub fn spawn_on_start(mut self, spawn: bool) -> Self {
        self.spawn_on_start = Some(spawn);
        self
    }

    pub fn working_directory(mut self, dir: impl Into<String>) -> Self {
        self.working_directory = Some(dir.into());
        self
    }

    pub fn build(self) -> SidecarConfig {
        SidecarConfig {
            sidecar_name: self.sidecar_name.unwrap_or_else(|| "main".into()),
            args: self.args,
            spawn_on_start: self.spawn_on_start.unwrap_or(true),
            working_directory: self.working_directory,
        }
    }
}
