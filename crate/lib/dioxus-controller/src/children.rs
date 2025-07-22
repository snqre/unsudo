use super::*;

pub type UseChildrenConfig = Vec<Element>;

pub type Children = Signal<Vec<Element>>;

pub fn use_children(init_cfg: UseChildrenConfig) -> Children {
    use_signal(|| init_cfg)
}