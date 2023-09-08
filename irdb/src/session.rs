pub trait Session {
    fn get_db(&self) -> Option<String>; 
    fn get_charset(&self) -> Option<String>;
    fn get_autocommit(&self) -> Option<String>;
}

pub trait SessionMut {
    fn set_db(&mut self, db: String);
    fn set_charset(&mut self, charset: String);
    fn set_autocommit(&mut self, autocommit: String);
}