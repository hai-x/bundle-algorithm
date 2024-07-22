use super::ModuleId;

#[derive(Debug, Clone, Copy)]
pub struct Dependency {
    pub(crate) is_async: bool,
    pub(crate) origin: ModuleId,
    pub(crate) target: ModuleId,
}
