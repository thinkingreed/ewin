#[derive(Debug, PartialEq, Default, Eq, Clone, Hash, Copy)]
pub enum DialogContType {
    #[default]
    AboutApp,
    FileProp(usize),
}
