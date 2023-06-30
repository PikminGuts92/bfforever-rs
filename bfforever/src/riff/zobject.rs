pub type FStringKey = u64;

pub trait ZObject {
    fn get_file_path(&self) -> FStringKey;
    fn set_file_path(&mut self, key: FStringKey);

    fn get_directory_path(&self) -> FStringKey;
    fn set_directory_path(&mut self, key: FStringKey);
}