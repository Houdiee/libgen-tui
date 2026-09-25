#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Focus {
    SearchBar,
    Table,
    PopupYes,
    PopupCancel,
    Nothing,
}
